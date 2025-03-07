// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
    collections::{BTreeMap, HashMap},
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use diesel::r2d2::R2D2Connection;
use iota_data_ingestion_core::Worker;
use iota_json_rpc_types::IotaMoveValue;
use iota_metrics::{get_metrics, spawn_monitored_task};
use iota_package_resolver::{PackageStore, PackageStoreWithLruCache, Resolver};
use iota_rest_api::{CheckpointData, CheckpointTransaction};
use iota_types::{
    base_types::ObjectID,
    dynamic_field::{DynamicFieldInfo, DynamicFieldName, DynamicFieldType},
    effects::TransactionEffectsAPI,
    event::SystemEpochInfoEventV1,
    iota_system_state::{
        IotaSystemStateTrait, get_iota_system_state,
        iota_system_state_summary::IotaSystemStateSummary,
    },
    messages_checkpoint::{
        CertifiedCheckpointSummary, CheckpointContents, CheckpointSequenceNumber,
    },
    object::{Object, Owner},
    transaction::TransactionDataAPI,
};
use itertools::Itertools;
use move_core_types::{
    annotated_value::{MoveStructLayout, MoveTypeLayout},
    language_storage::{StructTag, TypeTag},
};
use tap::tap::TapFallible;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

use crate::{
    db::ConnectionPool,
    errors::IndexerError,
    handlers::{
        CheckpointDataToCommit, EpochToCommit, TransactionObjectChangesToCommit,
        committer::start_tx_checkpoint_commit_task,
        tx_processor::{EpochEndIndexingObjectStore, IndexingPackageBuffer, TxChangesProcessor},
    },
    metrics::IndexerMetrics,
    models::display::StoredDisplay,
    store::{
        IndexerStore, PgIndexerStore,
        package_resolver::{IndexerStorePackageResolver, InterimPackageResolver},
    },
    types::{
        EventIndex, IndexedCheckpoint, IndexedDeletedObject, IndexedEpochInfo, IndexedEvent,
        IndexedObject, IndexedPackage, IndexedTransaction, IndexerResult, TransactionKind, TxIndex,
    },
};

const CHECKPOINT_QUEUE_SIZE: usize = 100;

pub async fn new_handlers<S, T>(
    state: S,
    metrics: IndexerMetrics,
    next_checkpoint_sequence_number: CheckpointSequenceNumber,
    cancel: CancellationToken,
) -> Result<CheckpointHandler<S, T>, IndexerError>
where
    S: IndexerStore + Clone + Sync + Send + 'static,
    T: R2D2Connection + 'static,
{
    let checkpoint_queue_size = std::env::var("CHECKPOINT_QUEUE_SIZE")
        .unwrap_or(CHECKPOINT_QUEUE_SIZE.to_string())
        .parse::<usize>()
        .unwrap();
    let global_metrics = get_metrics().unwrap();
    let (indexed_checkpoint_sender, indexed_checkpoint_receiver) =
        iota_metrics::metered_channel::channel(
            checkpoint_queue_size,
            &global_metrics
                .channel_inflight
                .with_label_values(&["checkpoint_indexing"]),
        );

    let state_clone = state.clone();
    let metrics_clone = metrics.clone();
    let (tx, package_tx) = watch::channel(None);
    spawn_monitored_task!(start_tx_checkpoint_commit_task(
        state_clone,
        metrics_clone,
        indexed_checkpoint_receiver,
        tx,
        next_checkpoint_sequence_number,
        cancel.clone()
    ));
    Ok(CheckpointHandler::new(
        state,
        metrics,
        indexed_checkpoint_sender,
        package_tx,
    ))
}

pub struct CheckpointHandler<S, T: R2D2Connection + 'static> {
    state: S,
    metrics: IndexerMetrics,
    indexed_checkpoint_sender: iota_metrics::metered_channel::Sender<CheckpointDataToCommit>,
    // buffers for packages that are being indexed but not committed to DB,
    // they will be periodically GCed to avoid OOM.
    package_buffer: Arc<Mutex<IndexingPackageBuffer>>,
    package_resolver: Arc<Resolver<PackageStoreWithLruCache<InterimPackageResolver<T>>>>,
}

#[async_trait]
impl<S, T> Worker for CheckpointHandler<S, T>
where
    S: IndexerStore + Clone + Sync + Send + 'static,
    T: R2D2Connection + 'static,
{
    async fn process_checkpoint(&self, checkpoint: CheckpointData) -> anyhow::Result<()> {
        self.metrics
            .latest_fullnode_checkpoint_sequence_number
            .set(checkpoint.checkpoint_summary.sequence_number as i64);
        let time_now_ms = chrono::Utc::now().timestamp_millis();
        let cp_download_lag = time_now_ms - checkpoint.checkpoint_summary.timestamp_ms as i64;
        info!(
            "checkpoint download lag for cp {}: {} ms",
            checkpoint.checkpoint_summary.sequence_number, cp_download_lag
        );
        self.metrics.download_lag_ms.set(cp_download_lag);
        self.metrics
            .max_downloaded_checkpoint_sequence_number
            .set(checkpoint.checkpoint_summary.sequence_number as i64);
        self.metrics
            .downloaded_checkpoint_timestamp_ms
            .set(checkpoint.checkpoint_summary.timestamp_ms as i64);
        info!(
            "Indexer lag: downloaded checkpoint {} with time now {} and checkpoint time {}",
            checkpoint.checkpoint_summary.sequence_number,
            time_now_ms,
            checkpoint.checkpoint_summary.timestamp_ms
        );
        let checkpoint_data = Self::index_checkpoint(
            self.state.clone().into(),
            checkpoint.clone(),
            Arc::new(self.metrics.clone()),
            Self::index_packages(&[checkpoint], &self.metrics),
            self.package_resolver.clone(),
        )
        .await?;
        self.indexed_checkpoint_sender.send(checkpoint_data).await?;
        Ok(())
    }

    fn preprocess_hook(&self, checkpoint: CheckpointData) -> anyhow::Result<()> {
        let package_objects = Self::get_package_objects(&[checkpoint]);
        self.package_buffer
            .lock()
            .unwrap()
            .insert_packages(package_objects);
        Ok(())
    }
}

impl<S, T> CheckpointHandler<S, T>
where
    S: IndexerStore + Clone + Sync + Send + 'static,
    T: R2D2Connection + 'static,
{
    fn new(
        state: S,
        metrics: IndexerMetrics,
        indexed_checkpoint_sender: iota_metrics::metered_channel::Sender<CheckpointDataToCommit>,
        package_tx: watch::Receiver<Option<CheckpointSequenceNumber>>,
    ) -> Self {
        let package_buffer = IndexingPackageBuffer::start(package_tx);
        let pg_blocking_cp = Self::pg_blocking_cp(state.clone()).unwrap();
        let package_db_resolver = IndexerStorePackageResolver::new(pg_blocking_cp);
        let in_mem_package_resolver = InterimPackageResolver::new(
            package_db_resolver,
            package_buffer.clone(),
            metrics.clone(),
        );
        let cached_package_resolver = PackageStoreWithLruCache::new(in_mem_package_resolver);
        let package_resolver = Arc::new(Resolver::new(cached_package_resolver));
        Self {
            state,
            metrics,
            indexed_checkpoint_sender,
            package_buffer,
            package_resolver,
        }
    }

    async fn index_epoch(
        state: Arc<S>,
        data: &CheckpointData,
    ) -> Result<Option<EpochToCommit>, IndexerError> {
        let checkpoint_object_store = EpochEndIndexingObjectStore::new(data);

        let CheckpointData {
            transactions,
            checkpoint_summary,
            checkpoint_contents: _,
        } = data;

        // Genesis epoch
        if *checkpoint_summary.sequence_number() == 0 {
            info!("Processing genesis epoch");
            let system_state: IotaSystemStateSummary =
                get_iota_system_state(&checkpoint_object_store)?.into_iota_system_state_summary();
            return Ok(Some(EpochToCommit {
                last_epoch: None,
                new_epoch: IndexedEpochInfo::from_new_system_state_summary(
                    system_state,
                    0, // first_checkpoint_id
                    None,
                ),
                network_total_transactions: 0,
            }));
        }

        // If not end of epoch, return
        if checkpoint_summary.end_of_epoch_data.is_none() {
            return Ok(None);
        }

        let system_state: IotaSystemStateSummary =
            get_iota_system_state(&checkpoint_object_store)?.into_iota_system_state_summary();

        let epoch_event = transactions
            .iter()
            .flat_map(|t| t.events.as_ref().map(|e| &e.data))
            .flatten()
            .find(|ev| ev.is_system_epoch_info_event())
            .unwrap_or_else(|| {
                panic!(
                    "Can't find SystemEpochInfoEventV1 in epoch end checkpoint {}",
                    checkpoint_summary.sequence_number()
                )
            });

        let event = bcs::from_bytes::<SystemEpochInfoEventV1>(&epoch_event.contents)?;

        // Now we just entered epoch X, we want to calculate the diff between
        // TotalTransactionsByEndOfEpoch(X-1) and TotalTransactionsByEndOfEpoch(X-2).
        // Note that on the indexer's chain-reading side, this is not guaranteed
        // to have the latest data. Rather than impose a wait on the reading
        // side, however, we overwrite this on the persisting side, where we can
        // guarantee that the previous epoch's checkpoints have been written to
        // db.

        let network_tx_count_prev_epoch = match system_state.epoch {
            // If first epoch change, this number is 0
            1 => Ok(0),
            _ => {
                let last_epoch = system_state.epoch - 2;
                state
                    .get_network_total_transactions_by_end_of_epoch(last_epoch)
                    .await
            }
        }?;

        Ok(Some(EpochToCommit {
            last_epoch: Some(IndexedEpochInfo::from_end_of_epoch_data(
                &system_state,
                checkpoint_summary,
                &event,
                network_tx_count_prev_epoch,
            )),
            new_epoch: IndexedEpochInfo::from_new_system_state_summary(
                system_state,
                checkpoint_summary.sequence_number + 1, // first_checkpoint_id
                Some(&event),
            ),
            network_total_transactions: checkpoint_summary.network_total_transactions,
        }))
    }

    async fn index_checkpoint(
        state: Arc<S>,
        data: CheckpointData,
        metrics: Arc<IndexerMetrics>,
        packages: Vec<IndexedPackage>,
        package_resolver: Arc<Resolver<impl PackageStore>>,
    ) -> Result<CheckpointDataToCommit, IndexerError> {
        let checkpoint_seq = data.checkpoint_summary.sequence_number;
        info!(checkpoint_seq, "Indexing checkpoint data blob");

        // Index epoch
        let epoch = Self::index_epoch(state, &data).await?;

        // Index Objects
        let object_changes: TransactionObjectChangesToCommit =
            Self::index_objects(data.clone(), &metrics, package_resolver.clone()).await?;
        let object_history_changes: TransactionObjectChangesToCommit =
            Self::index_objects_history(data.clone(), package_resolver.clone()).await?;

        let (checkpoint, db_transactions, db_events, db_tx_indices, db_event_indices, db_displays) = {
            let CheckpointData {
                transactions,
                checkpoint_summary,
                checkpoint_contents,
            } = data;

            let (db_transactions, db_events, db_tx_indices, db_event_indices, db_displays) =
                Self::index_transactions(
                    transactions,
                    &checkpoint_summary,
                    &checkpoint_contents,
                    &metrics,
                )
                .await?;

            let successful_tx_num: u64 = db_transactions.iter().map(|t| t.successful_tx_num).sum();
            (
                IndexedCheckpoint::from_iota_checkpoint(
                    &checkpoint_summary,
                    &checkpoint_contents,
                    successful_tx_num as usize,
                ),
                db_transactions,
                db_events,
                db_tx_indices,
                db_event_indices,
                db_displays,
            )
        };
        let time_now_ms = chrono::Utc::now().timestamp_millis();
        metrics
            .index_lag_ms
            .set(time_now_ms - checkpoint.timestamp_ms as i64);
        metrics
            .max_indexed_checkpoint_sequence_number
            .set(checkpoint.sequence_number as i64);
        metrics
            .indexed_checkpoint_timestamp_ms
            .set(checkpoint.timestamp_ms as i64);
        info!(
            "Indexer lag: indexed checkpoint {} with time now {} and checkpoint time {}",
            checkpoint.sequence_number, time_now_ms, checkpoint.timestamp_ms
        );

        Ok(CheckpointDataToCommit {
            checkpoint,
            transactions: db_transactions,
            events: db_events,
            tx_indices: db_tx_indices,
            event_indices: db_event_indices,
            display_updates: db_displays,
            object_changes,
            object_history_changes,
            packages,
            epoch,
        })
    }

    async fn index_transactions(
        transactions: Vec<CheckpointTransaction>,
        checkpoint_summary: &CertifiedCheckpointSummary,
        checkpoint_contents: &CheckpointContents,
        metrics: &IndexerMetrics,
    ) -> IndexerResult<(
        Vec<IndexedTransaction>,
        Vec<IndexedEvent>,
        Vec<TxIndex>,
        Vec<EventIndex>,
        BTreeMap<String, StoredDisplay>,
    )> {
        let checkpoint_seq = checkpoint_summary.sequence_number();

        let mut tx_seq_num_iter = checkpoint_contents
            .enumerate_transactions(checkpoint_summary)
            .map(|(seq, execution_digest)| (execution_digest.transaction, seq));

        if checkpoint_contents.size() != transactions.len() {
            return Err(IndexerError::FullNodeReading(format!(
                "CheckpointContents has different size {} compared to Transactions {} for checkpoint {}",
                checkpoint_contents.size(),
                transactions.len(),
                checkpoint_seq
            )));
        }

        let mut db_transactions = Vec::new();
        let mut db_events = Vec::new();
        let mut db_displays = BTreeMap::new();
        let mut db_tx_indices = Vec::new();
        let mut db_event_indices = Vec::new();

        for tx in transactions {
            let CheckpointTransaction {
                transaction: sender_signed_data,
                effects: fx,
                events,
                input_objects,
                output_objects,
            } = tx;
            // Unwrap safe - we checked they have equal length above
            let (tx_digest, tx_sequence_number) = tx_seq_num_iter.next().unwrap();
            if tx_digest != *sender_signed_data.digest() {
                return Err(IndexerError::FullNodeReading(format!(
                    "Transactions has different ordering from CheckpointContents, for checkpoint {}, Mismatch found at {} v.s. {}",
                    checkpoint_seq,
                    tx_digest,
                    sender_signed_data.digest()
                )));
            }

            let tx = sender_signed_data.transaction_data();
            let events = events
                .as_ref()
                .map(|events| events.data.clone())
                .unwrap_or_default();

            let transaction_kind = if tx.is_system_tx() {
                TransactionKind::SystemTransaction
            } else {
                TransactionKind::ProgrammableTransaction
            };

            db_events.extend(events.iter().enumerate().map(|(idx, event)| {
                IndexedEvent::from_event(
                    tx_sequence_number,
                    idx as u64,
                    *checkpoint_seq,
                    tx_digest,
                    event,
                    checkpoint_summary.timestamp_ms,
                )
            }));

            db_event_indices.extend(
                events.iter().enumerate().map(|(idx, event)| {
                    EventIndex::from_event(tx_sequence_number, idx as u64, event)
                }),
            );

            db_displays.extend(
                events
                    .iter()
                    .flat_map(StoredDisplay::try_from_event)
                    .map(|display| (display.object_type.clone(), display)),
            );

            let objects = input_objects
                .iter()
                .chain(output_objects.iter())
                .collect::<Vec<_>>();

            let (balance_change, object_changes) =
                TxChangesProcessor::new(&objects, metrics.clone())
                    .get_changes(tx, &fx, &tx_digest)
                    .await?;

            let db_txn = IndexedTransaction {
                tx_sequence_number,
                tx_digest,
                checkpoint_sequence_number: *checkpoint_summary.sequence_number(),
                timestamp_ms: checkpoint_summary.timestamp_ms,
                sender_signed_data: sender_signed_data.data().clone(),
                effects: fx.clone(),
                object_changes,
                balance_change,
                events,
                transaction_kind: transaction_kind.clone(),
                successful_tx_num: if fx.status().is_ok() {
                    tx.kind().tx_count() as u64
                } else {
                    0
                },
            };

            db_transactions.push(db_txn);

            // Input Objects
            let input_objects = tx
                .input_objects()
                .expect("committed txns have been validated")
                .into_iter()
                .map(|obj_kind| obj_kind.object_id())
                .collect::<Vec<_>>();

            // Changed Objects
            let changed_objects = fx
                .all_changed_objects()
                .into_iter()
                .map(|(object_ref, _owner, _write_kind)| object_ref.0)
                .collect::<Vec<_>>();

            // Payers
            let payers = vec![tx.gas_owner()];

            // Sender
            let sender = tx.sender();

            // Recipients
            let recipients = fx
                .all_changed_objects()
                .into_iter()
                .filter_map(|(_object_ref, owner, _write_kind)| match owner {
                    Owner::AddressOwner(address) => Some(address),
                    _ => None,
                })
                .unique()
                .collect::<Vec<_>>();

            // Move Calls
            let move_calls = tx
                .move_calls()
                .iter()
                .map(|(p, m, f)| (*<&ObjectID>::clone(p), m.to_string(), f.to_string()))
                .collect();

            db_tx_indices.push(TxIndex {
                tx_sequence_number,
                transaction_digest: tx_digest,
                checkpoint_sequence_number: *checkpoint_seq,
                input_objects,
                changed_objects,
                sender,
                payers,
                recipients,
                move_calls,
                tx_kind: transaction_kind,
            });
        }
        Ok((
            db_transactions,
            db_events,
            db_tx_indices,
            db_event_indices,
            db_displays,
        ))
    }

    pub(crate) async fn index_objects(
        data: CheckpointData,
        metrics: &IndexerMetrics,
        package_resolver: Arc<Resolver<impl PackageStore>>,
    ) -> Result<TransactionObjectChangesToCommit, IndexerError> {
        let _timer = metrics.indexing_objects_latency.start_timer();
        let checkpoint_seq = data.checkpoint_summary.sequence_number;

        let eventually_removed_object_refs_post_version =
            data.eventually_removed_object_refs_post_version();
        let indexed_eventually_removed_objects = eventually_removed_object_refs_post_version
            .into_iter()
            .map(|obj_ref| IndexedDeletedObject {
                object_id: obj_ref.0,
                object_version: obj_ref.1.into(),
                checkpoint_sequence_number: checkpoint_seq,
            })
            .collect();

        let latest_live_output_objects = data.latest_live_output_objects();
        let latest_live_output_object_map = latest_live_output_objects
            .clone()
            .into_iter()
            .map(|o| (o.id(), o.clone()))
            .collect::<HashMap<_, _>>();
        let move_struct_layout_map =
            get_move_struct_layout_map(latest_live_output_objects.clone(), package_resolver)
                .await?;
        let changed_objects = latest_live_output_objects
            .into_iter()
            .map(|o| {
                let df_info = try_create_dynamic_field_info(
                    o,
                    &move_struct_layout_map,
                    &latest_live_output_object_map,
                );
                df_info.map(|info| IndexedObject::from_object(checkpoint_seq, o.clone(), info))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(TransactionObjectChangesToCommit {
            changed_objects,
            deleted_objects: indexed_eventually_removed_objects,
        })
    }

    // similar to index_objects, but objects_history keeps all versions of objects
    async fn index_objects_history(
        data: CheckpointData,
        package_resolver: Arc<Resolver<impl PackageStore>>,
    ) -> Result<TransactionObjectChangesToCommit, IndexerError> {
        let checkpoint_seq = data.checkpoint_summary.sequence_number;
        let deleted_objects = data
            .transactions
            .iter()
            .flat_map(|tx| tx.removed_object_refs_post_version())
            .collect::<Vec<_>>();
        let indexed_deleted_objects: Vec<IndexedDeletedObject> = deleted_objects
            .into_iter()
            .map(|obj_ref| IndexedDeletedObject {
                object_id: obj_ref.0,
                object_version: obj_ref.1.into(),
                checkpoint_sequence_number: checkpoint_seq,
            })
            .collect();

        let latest_live_output_objects = data.latest_live_output_objects();
        let latest_live_output_object_map = latest_live_output_objects
            .clone()
            .into_iter()
            .map(|o| (o.id(), o.clone()))
            .collect::<HashMap<_, _>>();

        let output_objects = data
            .transactions
            .iter()
            .flat_map(|tx| &tx.output_objects)
            .collect::<Vec<_>>();
        // TODO(gegaowp): the current df_info implementation is not correct,
        // but we have decided remove all df_* except df_kind.
        let move_struct_layout_map =
            get_move_struct_layout_map(output_objects.clone(), package_resolver).await?;
        let changed_objects = output_objects
            .into_iter()
            .map(|o| {
                let df_info = try_create_dynamic_field_info(
                    o,
                    &move_struct_layout_map,
                    &latest_live_output_object_map,
                );
                df_info.map(|info| IndexedObject::from_object(checkpoint_seq, o.clone(), info))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(TransactionObjectChangesToCommit {
            changed_objects,
            deleted_objects: indexed_deleted_objects,
        })
    }

    fn index_packages(
        checkpoint_data: &[CheckpointData],
        metrics: &IndexerMetrics,
    ) -> Vec<IndexedPackage> {
        let _timer = metrics.indexing_packages_latency.start_timer();
        checkpoint_data
            .iter()
            .flat_map(|data| {
                let checkpoint_sequence_number = data.checkpoint_summary.sequence_number;
                data.transactions
                    .iter()
                    .flat_map(|tx| &tx.output_objects)
                    .filter_map(|o| {
                        if let iota_types::object::Data::Package(p) = &o.data {
                            Some(IndexedPackage {
                                package_id: o.id(),
                                move_package: p.clone(),
                                checkpoint_sequence_number,
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    pub(crate) fn get_package_objects(
        checkpoint_data: &[CheckpointData],
    ) -> Vec<(IndexedPackage, Object)> {
        checkpoint_data
            .iter()
            .flat_map(|data| {
                let checkpoint_sequence_number = data.checkpoint_summary.sequence_number;
                data.transactions
                    .iter()
                    .flat_map(|tx| &tx.output_objects)
                    .filter_map(|o| {
                        if let iota_types::object::Data::Package(p) = &o.data {
                            let indexed_pkg = IndexedPackage {
                                package_id: o.id(),
                                move_package: p.clone(),
                                checkpoint_sequence_number,
                            };
                            Some((indexed_pkg, o.clone()))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    pub(crate) fn pg_blocking_cp(state: S) -> Result<ConnectionPool<T>, IndexerError> {
        let state_as_any = state.as_any();
        if let Some(pg_state) = state_as_any.downcast_ref::<PgIndexerStore<T>>() {
            return Ok(pg_state.blocking_cp());
        }
        Err(IndexerError::Uncategorized(anyhow::anyhow!(
            "Failed to downcast state to PgIndexerStore"
        )))
    }
}

async fn get_move_struct_layout_map(
    objects: Vec<&Object>,
    package_resolver: Arc<Resolver<impl PackageStore>>,
) -> Result<HashMap<StructTag, MoveStructLayout>, IndexerError> {
    let struct_tags = objects
        .into_iter()
        .filter_map(|o| {
            let move_object = o.data.try_as_move().cloned();
            move_object.map(|move_object| {
                let struct_tag: StructTag = move_object.type_().clone().into();
                struct_tag
            })
        })
        .collect::<Vec<_>>();
    let struct_tags = struct_tags.into_iter().unique().collect::<Vec<_>>();
    info!(
        "Resolving Move struct layouts for struct tags of size {}.",
        struct_tags.len()
    );
    let move_struct_layout_futures = struct_tags
        .into_iter()
        .map(|struct_tag| {
            let package_resolver_clone = package_resolver.clone();
            async move {
                let move_type_layout = package_resolver_clone
                    .type_layout(TypeTag::Struct(Box::new(struct_tag.clone())))
                    .await
                    .map_err(|e| {
                        IndexerError::DynamicField(format!(
                            "Fail to resolve struct layout for {:?} with {:?}.",
                            struct_tag, e
                        ))
                    })?;
                let move_struct_layout = match move_type_layout {
                    MoveTypeLayout::Struct(s) => Ok(s),
                    _ => Err(IndexerError::ResolveMoveStruct(
                        "MoveTypeLayout is not Struct".to_string(),
                    )),
                }?;
                Ok::<
                    (
                        move_core_types::language_storage::StructTag,
                        move_core_types::annotated_value::MoveStructLayout,
                    ),
                    IndexerError,
                >((struct_tag, move_struct_layout))
            }
        })
        .collect::<Vec<_>>();
    let move_struct_layout_map = futures::future::try_join_all(move_struct_layout_futures)
        .await?
        .into_iter()
        .collect::<HashMap<_, _>>();
    Ok(move_struct_layout_map)
}

fn try_create_dynamic_field_info(
    o: &Object,
    struct_tag_to_move_struct_layout: &HashMap<StructTag, MoveStructLayout>,
    latest_objects: &HashMap<ObjectID, Object>,
) -> IndexerResult<Option<DynamicFieldInfo>> {
    // Skip if not a move object
    let Some(move_object) = o.data.try_as_move().cloned() else {
        return Ok(None);
    };

    if !move_object.type_().is_dynamic_field() {
        return Ok(None);
    }

    let struct_tag: StructTag = move_object.type_().clone().into();
    let move_struct_layout = struct_tag_to_move_struct_layout
        .get(&struct_tag)
        .cloned()
        .ok_or_else(|| {
            IndexerError::DynamicField(format!(
                "Cannot find struct layout in mapfor {:?}.",
                struct_tag
            ))
        })?;
    let move_struct = move_object.to_move_struct(&move_struct_layout)?;
    let (name_value, type_, object_id) =
        DynamicFieldInfo::parse_move_object(&move_struct).tap_err(|e| warn!("{e}"))?;
    let name_type = move_object.type_().try_extract_field_name(&type_)?;
    let bcs_name = bcs::to_bytes(&name_value.clone().undecorate()).map_err(|e| {
        IndexerError::Serde(format!(
            "Failed to serialize dynamic field name {:?}: {e}",
            name_value
        ))
    })?;
    let name = DynamicFieldName {
        type_: name_type,
        value: IotaMoveValue::from(name_value).to_json_value(),
    };
    Ok(Some(match type_ {
        DynamicFieldType::DynamicObject => {
            let object = latest_objects
                .get(&object_id)
                .ok_or(IndexerError::Uncategorized(anyhow::anyhow!(
                    "Failed to find object_id {:?} when trying to create dynamic field info",
                    object_id
                )))?;
            let version = object.version();
            let digest = object.digest();
            let object_type = object.data.type_().unwrap().clone();
            DynamicFieldInfo {
                name,
                bcs_name,
                type_,
                object_type: object_type.to_canonical_string(/* with_prefix */ true),
                object_id,
                version,
                digest,
            }
        }
        DynamicFieldType::DynamicField => DynamicFieldInfo {
            name,
            bcs_name,
            type_,
            object_type: move_object.into_type().into_type_params()[1]
                .to_canonical_string(/* with_prefix */ true),
            object_id: o.id(),
            version: o.version(),
            digest: o.digest(),
        },
    }))
}
