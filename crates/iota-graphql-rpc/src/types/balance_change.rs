// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_graphql::*;
use iota_json_rpc_types::BalanceChange as StoredBalanceChange;
use iota_types::object::Owner as NativeOwner;

use crate::{
    error::Error,
    types::{big_int::BigInt, iota_address::IotaAddress, move_type::MoveType, owner::Owner},
};

pub(crate) struct BalanceChange {
    stored: StoredBalanceChange,
    /// The checkpoint sequence number this was viewed at.
    checkpoint_viewed_at: u64,
}

/// Effects to the balance (sum of coin values per coin type) owned by an
/// address or object.
#[Object]
impl BalanceChange {
    /// The address or object whose balance has changed.
    async fn owner(&self) -> Option<Owner> {
        use NativeOwner as O;

        match self.stored.owner {
            O::AddressOwner(addr) | O::ObjectOwner(addr) => Some(Owner {
                address: IotaAddress::from(addr),
                checkpoint_viewed_at: self.checkpoint_viewed_at,
                root_version: None,
            }),

            O::Shared { .. } | O::Immutable => None,
        }
    }

    /// The inner type of the coin whose balance has changed (e.g.
    /// `0x2::iota::IOTA`).
    async fn coin_type(&self) -> Option<MoveType> {
        Some(MoveType::new(self.stored.coin_type.clone()))
    }

    /// The signed balance change.
    async fn amount(&self) -> Option<BigInt> {
        Some(BigInt::from(self.stored.amount))
    }
}

impl BalanceChange {
    /// `checkpoint_viewed_at` represents the checkpoint sequence number at
    /// which this `BalanceChange` was queried for. This is stored on
    /// `BalanceChange` so that when viewing that entity's state, it will be
    /// as if it was read at the same checkpoint.
    pub(crate) fn read(bytes: &[u8], checkpoint_viewed_at: u64) -> Result<Self, Error> {
        let stored = bcs::from_bytes(bytes)
            .map_err(|e| Error::Internal(format!("Error deserializing BalanceChange: {e}")))?;

        Ok(Self {
            stored,
            checkpoint_viewed_at,
        })
    }
}
