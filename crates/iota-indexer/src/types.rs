// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::errors::IndexerError;
use move_core_types::language_storage::StructTag;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use iota_json_rpc_types::{
    ObjectChange, IotaTransactionBlockResponse, IotaTransactionBlockResponseOptions,
};
use iota_types::base_types::{ObjectDigest, SequenceNumber};
use iota_types::base_types::{ObjectID, IotaAddress};
use iota_types::crypto::AggregateAuthoritySignature;
use iota_types::digests::TransactionDigest;
use iota_types::dynamic_field::DynamicFieldInfo;
use iota_types::effects::TransactionEffects;
use iota_types::event::SystemEpochInfoEvent;
use iota_types::messages_checkpoint::{
    CertifiedCheckpointSummary, CheckpointCommitment, CheckpointDigest, CheckpointSequenceNumber,
    EndOfEpochData,
};
use iota_types::move_package::MovePackage;
use iota_types::object::{Object, Owner};
use iota_types::iota_serde::IotaStructTag;
use iota_types::iota_system_state::iota_system_state_summary::IotaSystemStateSummary;
use iota_types::transaction::SenderSignedData;

pub type IndexerResult<T> = Result<T, IndexerError>;

#[derive(Debug)]
pub struct IndexedCheckpoint {
    pub sequence_number: u64,
    pub checkpoint_digest: CheckpointDigest,
    pub epoch: u64,
    pub tx_digests: Vec<TransactionDigest>,
    pub network_total_transactions: u64,
    pub previous_checkpoint_digest: Option<CheckpointDigest>,
    pub timestamp_ms: u64,
    pub total_gas_cost: i64, // total gas cost could be negative
    pub computation_cost: u64,
    pub storage_cost: u64,
    pub storage_rebate: u64,
    pub non_refundable_storage_fee: u64,
    pub checkpoint_commitments: Vec<CheckpointCommitment>,
    pub validator_signature: AggregateAuthoritySignature,
    pub successful_tx_num: usize,
    pub end_of_epoch_data: Option<EndOfEpochData>,
    pub end_of_epoch: bool,
    pub min_tx_sequence_number: u64,
    pub max_tx_sequence_number: u64,
}

impl IndexedCheckpoint {
    pub fn from_iota_checkpoint(
        checkpoint: &iota_types::messages_checkpoint::CertifiedCheckpointSummary,
        contents: &iota_types::messages_checkpoint::CheckpointContents,
        successful_tx_num: usize,
    ) -> Self {
        let total_gas_cost = checkpoint.epoch_rolling_gas_cost_summary.computation_cost as i64
            + checkpoint.epoch_rolling_gas_cost_summary.storage_cost as i64
            - checkpoint.epoch_rolling_gas_cost_summary.storage_rebate as i64;
        let tx_digests = contents.iter().map(|t| t.transaction).collect::<Vec<_>>();
        let max_tx_sequence_number = checkpoint.network_total_transactions - 1;
        // NOTE: + 1u64 first to avoid subtraction with overflow
        let min_tx_sequence_number = max_tx_sequence_number + 1u64 - tx_digests.len() as u64;
        let auth_sig = &checkpoint.auth_sig().signature;
        Self {
            sequence_number: checkpoint.sequence_number,
            checkpoint_digest: *checkpoint.digest(),
            epoch: checkpoint.epoch,
            tx_digests,
            previous_checkpoint_digest: checkpoint.previous_digest,
            end_of_epoch_data: checkpoint.end_of_epoch_data.clone(),
            end_of_epoch: checkpoint.end_of_epoch_data.clone().is_some(),
            total_gas_cost,
            computation_cost: checkpoint.epoch_rolling_gas_cost_summary.computation_cost,
            storage_cost: checkpoint.epoch_rolling_gas_cost_summary.storage_cost,
            storage_rebate: checkpoint.epoch_rolling_gas_cost_summary.storage_rebate,
            non_refundable_storage_fee: checkpoint
                .epoch_rolling_gas_cost_summary
                .non_refundable_storage_fee,
            successful_tx_num,
            network_total_transactions: checkpoint.network_total_transactions,
            timestamp_ms: checkpoint.timestamp_ms,
            validator_signature: auth_sig.clone(),
            checkpoint_commitments: checkpoint.checkpoint_commitments.clone(),
            min_tx_sequence_number,
            max_tx_sequence_number,
        }
    }
}

/// Represents system state and summary info at the start and end of an epoch. Optional fields are
/// populated at epoch boundary, since they cannot be determined at the start of the epoch.
#[derive(Clone, Debug, Default)]
pub struct IndexedEpochInfo {
    pub epoch: u64,
    pub first_checkpoint_id: u64,
    pub epoch_start_timestamp: u64,
    pub reference_gas_price: u64,
    pub protocol_version: u64,
    pub total_stake: u64,
    pub storage_fund_balance: u64,
    pub system_state: Vec<u8>,
    pub epoch_total_transactions: Option<u64>,
    pub last_checkpoint_id: Option<u64>,
    pub epoch_end_timestamp: Option<u64>,
    pub storage_fund_reinvestment: Option<u64>,
    pub storage_charge: Option<u64>,
    pub storage_rebate: Option<u64>,
    pub stake_subsidy_amount: Option<u64>,
    pub total_gas_fees: Option<u64>,
    pub total_stake_rewards_distributed: Option<u64>,
    pub leftover_storage_fund_inflow: Option<u64>,
    pub epoch_commitments: Option<Vec<CheckpointCommitment>>,
}

impl IndexedEpochInfo {
    pub fn from_new_system_state_summary(
        new_system_state_summary: IotaSystemStateSummary,
        first_checkpoint_id: u64,
        event: Option<&SystemEpochInfoEvent>,
    ) -> IndexedEpochInfo {
        Self {
            epoch: new_system_state_summary.epoch,
            first_checkpoint_id,
            epoch_start_timestamp: new_system_state_summary.epoch_start_timestamp_ms,
            reference_gas_price: new_system_state_summary.reference_gas_price,
            protocol_version: new_system_state_summary.protocol_version,
            // NOTE: total_stake and storage_fund_balance are about new epoch,
            // although the event is generated at the end of the previous epoch,
            // the event is optional b/c no such event for the first epoch.
            total_stake: event.map(|e| e.total_stake).unwrap_or(0),
            storage_fund_balance: event.map(|e| e.storage_fund_balance).unwrap_or(0),
            system_state: bcs::to_bytes(&new_system_state_summary).unwrap(),
            ..Default::default()
        }
    }

    /// Creates `IndexedEpochInfo` for epoch X-1 at the boundary of epoch X-1 to X.
    /// `network_total_tx_num_at_last_epoch_end` is needed to determine the number of transactions
    /// that occurred in the epoch X-1.
    pub fn from_end_of_epoch_data(
        system_state_summary: &IotaSystemStateSummary,
        last_checkpoint_summary: &CertifiedCheckpointSummary,
        event: &SystemEpochInfoEvent,
        network_total_tx_num_at_last_epoch_end: u64,
    ) -> IndexedEpochInfo {
        Self {
            epoch: last_checkpoint_summary.epoch,
            epoch_total_transactions: Some(
                last_checkpoint_summary.network_total_transactions
                    - network_total_tx_num_at_last_epoch_end,
            ),
            last_checkpoint_id: Some(*last_checkpoint_summary.sequence_number()),
            epoch_end_timestamp: Some(last_checkpoint_summary.timestamp_ms),
            storage_fund_reinvestment: Some(event.storage_fund_reinvestment),
            storage_charge: Some(event.storage_charge),
            storage_rebate: Some(event.storage_rebate),
            leftover_storage_fund_inflow: Some(event.leftover_storage_fund_inflow),
            stake_subsidy_amount: Some(event.stake_subsidy_amount),
            total_gas_fees: Some(event.total_gas_fees),
            total_stake_rewards_distributed: Some(event.total_stake_rewards_distributed),
            epoch_commitments: last_checkpoint_summary
                .end_of_epoch_data
                .as_ref()
                .map(|e| e.epoch_commitments.clone()),
            system_state: bcs::to_bytes(system_state_summary).unwrap(),
            // The following felds will not and shall not be upserted
            // into DB. We have them below to make compiler and diesel happy
            first_checkpoint_id: 0,
            epoch_start_timestamp: 0,
            reference_gas_price: 0,
            protocol_version: 0,
            total_stake: 0,
            storage_fund_balance: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IndexedEvent {
    pub tx_sequence_number: u64,
    pub event_sequence_number: u64,
    pub checkpoint_sequence_number: u64,
    pub transaction_digest: TransactionDigest,
    pub senders: Vec<IotaAddress>,
    pub package: ObjectID,
    pub module: String,
    pub event_type: String,
    pub event_type_package: ObjectID,
    pub event_type_module: String,
    /// Struct name of the event, without type parameters.
    pub event_type_name: String,
    pub bcs: Vec<u8>,
    pub timestamp_ms: u64,
}

impl IndexedEvent {
    pub fn from_event(
        tx_sequence_number: u64,
        event_sequence_number: u64,
        checkpoint_sequence_number: u64,
        transaction_digest: TransactionDigest,
        event: &iota_types::event::Event,
        timestamp_ms: u64,
    ) -> Self {
        Self {
            tx_sequence_number,
            event_sequence_number,
            checkpoint_sequence_number,
            transaction_digest,
            senders: vec![event.sender],
            package: event.package_id,
            module: event.transaction_module.to_string(),
            event_type: event.type_.to_canonical_string(/* with_prefix */ true),
            event_type_package: event.type_.address.into(),
            event_type_module: event.type_.module.to_string(),
            event_type_name: event.type_.name.to_string(),
            bcs: event.contents.clone(),
            timestamp_ms,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventIndex {
    pub tx_sequence_number: u64,
    pub event_sequence_number: u64,
    pub sender: IotaAddress,
    pub emit_package: ObjectID,
    pub emit_module: String,
    pub type_package: ObjectID,
    pub type_module: String,
    /// Struct name of the event, without type parameters.
    pub type_name: String,
    /// Type instantiation of the event, with type name and type parameters, if any.
    pub type_instantiation: String,
}

impl EventIndex {
    pub fn from_event(
        tx_sequence_number: u64,
        event_sequence_number: u64,
        event: &iota_types::event::Event,
    ) -> Self {
        let type_instantiation = event
            .type_
            .to_canonical_string(/* with_prefix */ true)
            .splitn(3, "::")
            .collect::<Vec<_>>()[2]
            .to_string();
        Self {
            tx_sequence_number,
            event_sequence_number,
            sender: event.sender,
            emit_package: event.package_id,
            emit_module: event.transaction_module.to_string(),
            type_package: event.type_.address.into(),
            type_module: event.type_.module.to_string(),
            type_name: event.type_.name.to_string(),
            type_instantiation,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum OwnerType {
    Immutable = 0,
    Address = 1,
    Object = 2,
    Shared = 3,
}

pub enum ObjectStatus {
    Active = 0,
    WrappedOrDeleted = 1,
}

impl TryFrom<i16> for ObjectStatus {
    type Error = IndexerError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => ObjectStatus::Active,
            1 => ObjectStatus::WrappedOrDeleted,
            value => {
                return Err(IndexerError::PersistentStorageDataCorruptionError(format!(
                    "{value} as ObjectStatus"
                )))
            }
        })
    }
}

impl TryFrom<i16> for OwnerType {
    type Error = IndexerError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => OwnerType::Immutable,
            1 => OwnerType::Address,
            2 => OwnerType::Object,
            3 => OwnerType::Shared,
            value => {
                return Err(IndexerError::PersistentStorageDataCorruptionError(format!(
                    "{value} as OwnerType"
                )))
            }
        })
    }
}

// Returns owner_type, owner_address
pub fn owner_to_owner_info(owner: &Owner) -> (OwnerType, Option<IotaAddress>) {
    match owner {
        Owner::AddressOwner(address) => (OwnerType::Address, Some(*address)),
        Owner::ObjectOwner(address) => (OwnerType::Object, Some(*address)),
        Owner::Shared { .. } => (OwnerType::Shared, None),
        Owner::Immutable => (OwnerType::Immutable, None),
    }
}

#[derive(Debug, Copy, Clone)]
pub enum DynamicFieldKind {
    DynamicField = 0,
    DynamicObject = 1,
}

#[derive(Clone, Debug)]
pub struct IndexedObject {
    pub checkpoint_sequence_number: CheckpointSequenceNumber,
    pub object: Object,
    pub df_info: Option<DynamicFieldInfo>,
}

impl IndexedObject {
    pub fn from_object(
        checkpoint_sequence_number: CheckpointSequenceNumber,
        object: Object,
        df_info: Option<DynamicFieldInfo>,
    ) -> Self {
        Self {
            checkpoint_sequence_number,
            object,
            df_info,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IndexedDeletedObject {
    pub object_id: ObjectID,
    pub object_version: u64,
    pub checkpoint_sequence_number: u64,
}

#[derive(Debug)]
pub struct IndexedPackage {
    pub package_id: ObjectID,
    pub move_package: MovePackage,
    pub checkpoint_sequence_number: u64,
}

#[derive(Debug, Clone)]
pub enum TransactionKind {
    SystemTransaction = 0,
    ProgrammableTransaction = 1,
}

#[derive(Debug, Clone)]
pub struct IndexedTransaction {
    pub tx_sequence_number: u64,
    pub tx_digest: TransactionDigest,
    pub sender_signed_data: SenderSignedData,
    pub effects: TransactionEffects,
    pub checkpoint_sequence_number: u64,
    pub timestamp_ms: u64,
    pub object_changes: Vec<IndexedObjectChange>,
    pub balance_change: Vec<iota_json_rpc_types::BalanceChange>,
    pub events: Vec<iota_types::event::Event>,
    pub transaction_kind: TransactionKind,
    pub successful_tx_num: u64,
}

#[derive(Debug, Clone)]
pub struct TxIndex {
    pub tx_sequence_number: u64,
    pub tx_kind: TransactionKind,
    pub transaction_digest: TransactionDigest,
    pub checkpoint_sequence_number: u64,
    pub input_objects: Vec<ObjectID>,
    pub changed_objects: Vec<ObjectID>,
    pub payers: Vec<IotaAddress>,
    pub sender: IotaAddress,
    pub recipients: Vec<IotaAddress>,
    pub move_calls: Vec<(ObjectID, String, String)>,
}

// ObjectChange is not bcs deserializable, IndexedObjectChange is.
#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum IndexedObjectChange {
    Published {
        package_id: ObjectID,
        version: SequenceNumber,
        digest: ObjectDigest,
        modules: Vec<String>,
    },
    Transferred {
        sender: IotaAddress,
        recipient: Owner,
        #[serde_as(as = "IotaStructTag")]
        object_type: StructTag,
        object_id: ObjectID,
        version: SequenceNumber,
        digest: ObjectDigest,
    },
    /// Object mutated.
    Mutated {
        sender: IotaAddress,
        owner: Owner,
        #[serde_as(as = "IotaStructTag")]
        object_type: StructTag,
        object_id: ObjectID,
        version: SequenceNumber,
        previous_version: SequenceNumber,
        digest: ObjectDigest,
    },
    /// Delete object
    Deleted {
        sender: IotaAddress,
        #[serde_as(as = "IotaStructTag")]
        object_type: StructTag,
        object_id: ObjectID,
        version: SequenceNumber,
    },
    /// Wrapped object
    Wrapped {
        sender: IotaAddress,
        #[serde_as(as = "IotaStructTag")]
        object_type: StructTag,
        object_id: ObjectID,
        version: SequenceNumber,
    },
    /// New object creation
    Created {
        sender: IotaAddress,
        owner: Owner,
        #[serde_as(as = "IotaStructTag")]
        object_type: StructTag,
        object_id: ObjectID,
        version: SequenceNumber,
        digest: ObjectDigest,
    },
}

impl From<ObjectChange> for IndexedObjectChange {
    fn from(oc: ObjectChange) -> Self {
        match oc {
            ObjectChange::Published {
                package_id,
                version,
                digest,
                modules,
            } => Self::Published {
                package_id,
                version,
                digest,
                modules,
            },
            ObjectChange::Transferred {
                sender,
                recipient,
                object_type,
                object_id,
                version,
                digest,
            } => Self::Transferred {
                sender,
                recipient,
                object_type,
                object_id,
                version,
                digest,
            },
            ObjectChange::Mutated {
                sender,
                owner,
                object_type,
                object_id,
                version,
                previous_version,
                digest,
            } => Self::Mutated {
                sender,
                owner,
                object_type,
                object_id,
                version,
                previous_version,
                digest,
            },
            ObjectChange::Deleted {
                sender,
                object_type,
                object_id,
                version,
            } => Self::Deleted {
                sender,
                object_type,
                object_id,
                version,
            },
            ObjectChange::Wrapped {
                sender,
                object_type,
                object_id,
                version,
            } => Self::Wrapped {
                sender,
                object_type,
                object_id,
                version,
            },
            ObjectChange::Created {
                sender,
                owner,
                object_type,
                object_id,
                version,
                digest,
            } => Self::Created {
                sender,
                owner,
                object_type,
                object_id,
                version,
                digest,
            },
        }
    }
}

impl From<IndexedObjectChange> for ObjectChange {
    fn from(val: IndexedObjectChange) -> Self {
        match val {
            IndexedObjectChange::Published {
                package_id,
                version,
                digest,
                modules,
            } => ObjectChange::Published {
                package_id,
                version,
                digest,
                modules,
            },
            IndexedObjectChange::Transferred {
                sender,
                recipient,
                object_type,
                object_id,
                version,
                digest,
            } => ObjectChange::Transferred {
                sender,
                recipient,
                object_type,
                object_id,
                version,
                digest,
            },
            IndexedObjectChange::Mutated {
                sender,
                owner,
                object_type,
                object_id,
                version,
                previous_version,
                digest,
            } => ObjectChange::Mutated {
                sender,
                owner,
                object_type,
                object_id,
                version,
                previous_version,
                digest,
            },
            IndexedObjectChange::Deleted {
                sender,
                object_type,
                object_id,
                version,
            } => ObjectChange::Deleted {
                sender,
                object_type,
                object_id,
                version,
            },
            IndexedObjectChange::Wrapped {
                sender,
                object_type,
                object_id,
                version,
            } => ObjectChange::Wrapped {
                sender,
                object_type,
                object_id,
                version,
            },
            IndexedObjectChange::Created {
                sender,
                owner,
                object_type,
                object_id,
                version,
                digest,
            } => ObjectChange::Created {
                sender,
                owner,
                object_type,
                object_id,
                version,
                digest,
            },
        }
    }
}

// IotaTransactionBlockResponseWithOptions is only used on the reading path
pub struct IotaTransactionBlockResponseWithOptions {
    pub response: IotaTransactionBlockResponse,
    pub options: IotaTransactionBlockResponseOptions,
}

impl From<IotaTransactionBlockResponseWithOptions> for IotaTransactionBlockResponse {
    fn from(value: IotaTransactionBlockResponseWithOptions) -> Self {
        let IotaTransactionBlockResponseWithOptions { response, options } = value;

        IotaTransactionBlockResponse {
            digest: response.digest,
            transaction: options.show_input.then_some(response.transaction).flatten(),
            raw_transaction: options
                .show_raw_input
                .then_some(response.raw_transaction)
                .unwrap_or_default(),
            effects: options.show_effects.then_some(response.effects).flatten(),
            events: options.show_events.then_some(response.events).flatten(),
            object_changes: options
                .show_object_changes
                .then_some(response.object_changes)
                .flatten(),
            balance_changes: options
                .show_balance_changes
                .then_some(response.balance_changes)
                .flatten(),
            timestamp_ms: response.timestamp_ms,
            confirmed_local_execution: response.confirmed_local_execution,
            checkpoint: response.checkpoint,
            errors: vec![],
            raw_effects: options
                .show_raw_effects
                .then_some(response.raw_effects)
                .unwrap_or_default(),
        }
    }
}
