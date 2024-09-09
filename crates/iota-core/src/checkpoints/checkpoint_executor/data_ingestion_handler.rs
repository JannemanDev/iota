// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::checkpoints::CheckpointStore;
use crate::execution_cache::{ObjectCacheRead, TransactionCacheRead};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use iota_storage::blob::{Blob, BlobEncoding};
use iota_types::digests::TransactionDigest;
use iota_types::effects::TransactionEffectsAPI;
use iota_types::error::{IotaError, IotaResult, UserInputError};
use iota_types::full_checkpoint_content::{CheckpointData, CheckpointTransaction};
use iota_types::messages_checkpoint::VerifiedCheckpoint;
use iota_types::storage::ObjectKey;

pub(crate) fn load_checkpoint_data(
    checkpoint: VerifiedCheckpoint,
    object_cache_reader: &dyn ObjectCacheRead,
    transaction_cache_reader: &dyn TransactionCacheRead,
    checkpoint_store: Arc<CheckpointStore>,
    transaction_digests: &[TransactionDigest],
) -> IotaResult<CheckpointData> {
    let checkpoint_contents = checkpoint_store
        .get_checkpoint_contents(&checkpoint.content_digest)?
        .expect("checkpoint content has to be stored");

    let transactions = transaction_cache_reader
        .multi_get_transaction_blocks(transaction_digests)?
        .into_iter()
        .zip(transaction_digests)
        .map(|(tx, digest)| tx.ok_or(IotaError::TransactionNotFound { digest: *digest }))
        .collect::<IotaResult<Vec<_>>>()?;

    let effects = transaction_cache_reader
        .multi_get_executed_effects(transaction_digests)?
        .into_iter()
        .zip(transaction_digests)
        .map(|(effects, &digest)| effects.ok_or(IotaError::TransactionNotFound { digest }))
        .collect::<IotaResult<Vec<_>>>()?;

    let event_digests = effects
        .iter()
        .flat_map(|fx| fx.events_digest().copied())
        .collect::<Vec<_>>();

    let events = transaction_cache_reader
        .multi_get_events(&event_digests)?
        .into_iter()
        .zip(&event_digests)
        .map(|(event, digest)| event.ok_or(IotaError::TransactionEventsNotFound { digest: *digest }))
        .collect::<IotaResult<Vec<_>>>()?;

    let events: HashMap<_, _> = event_digests.into_iter().zip(events).collect();
    let mut full_transactions = Vec::with_capacity(transactions.len());
    for (tx, fx) in transactions.into_iter().zip(effects) {
        let events = fx.events_digest().map(|event_digest| {
            events
                .get(event_digest)
                .cloned()
                .expect("event was already checked to be present")
        });

        let input_object_keys = fx
            .modified_at_versions()
            .into_iter()
            .map(|(object_id, version)| ObjectKey(object_id, version))
            .collect::<Vec<_>>();

        let input_objects = object_cache_reader
            .multi_get_objects_by_key(&input_object_keys)?
            .into_iter()
            .zip(&input_object_keys)
            .map(|(object, object_key)| {
                object.ok_or(IotaError::UserInputError {
                    error: UserInputError::ObjectNotFound {
                        object_id: object_key.0,
                        version: Some(object_key.1),
                    },
                })
            })
            .collect::<IotaResult<Vec<_>>>()?;

        let output_object_keys = fx
            .all_changed_objects()
            .into_iter()
            .map(|(object_ref, _owner, _kind)| ObjectKey::from(object_ref))
            .collect::<Vec<_>>();

        let output_objects = object_cache_reader
            .multi_get_objects_by_key(&output_object_keys)?
            .into_iter()
            .zip(&output_object_keys)
            .map(|(object, object_key)| {
                object.ok_or(IotaError::UserInputError {
                    error: UserInputError::ObjectNotFound {
                        object_id: object_key.0,
                        version: Some(object_key.1),
                    },
                })
            })
            .collect::<IotaResult<Vec<_>>>()?;

        let full_transaction = CheckpointTransaction {
            transaction: (*tx).clone().into(),
            effects: fx,
            events,
            input_objects,
            output_objects,
        };
        full_transactions.push(full_transaction);
    }
    let checkpoint_data = CheckpointData {
        checkpoint_summary: checkpoint.into(),
        checkpoint_contents,
        transactions: full_transactions,
    };
    Ok(checkpoint_data)
}

pub(crate) fn store_checkpoint_locally(
    path: PathBuf,
    checkpoint_data: &CheckpointData,
) -> IotaResult {
    let file_name = format!("{}.chk", checkpoint_data.checkpoint_summary.sequence_number);

    std::fs::create_dir_all(&path).map_err(|err| {
        IotaError::FileIOError(format!(
            "failed to save full checkpoint content locally {:?}",
            err
        ))
    })?;

    Blob::encode(&checkpoint_data, BlobEncoding::Bcs)
        .map_err(|_| IotaError::TransactionSerializationError {
            error: "failed to serialize full checkpoint content".to_string(),
        }) // Map the first error
        .and_then(|blob| {
            std::fs::write(path.join(file_name), blob.to_bytes()).map_err(|_| {
                IotaError::FileIOError("failed to save full checkpoint content locally".to_string())
            })
        })?;

    Ok(())
}
