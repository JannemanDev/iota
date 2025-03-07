// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
// @generated automatically by Diesel CLI.

diesel::table! {
    progress_store (task_name) {
        task_name -> Text,
        checkpoint -> Int8,
        target_checkpoint -> Int8,
        timestamp -> Nullable<Timestamp>,
    }
}

diesel::table! {
    iota_progress_store (id) {
        id -> Int4,
        txn_digest -> Bytea,
    }
}

diesel::table! {
    iota_error_transactions (txn_digest) {
        txn_digest -> Bytea,
        sender_address -> Bytea,
        timestamp_ms -> Int8,
        failure_status -> Text,
        cmd_idx -> Nullable<Int8>,
    }
}

diesel::table! {
    token_transfer (chain_id, nonce, status) {
        chain_id -> Int4,
        nonce -> Int8,
        status -> Text,
        block_height -> Int8,
        timestamp_ms -> Int8,
        txn_hash -> Bytea,
        txn_sender -> Bytea,
        gas_usage -> Int8,
        data_source -> Text,
    }
}

diesel::table! {
    token_transfer_data (chain_id, nonce) {
        chain_id -> Int4,
        nonce -> Int8,
        block_height -> Int8,
        timestamp_ms -> Int8,
        txn_hash -> Bytea,
        sender_address -> Bytea,
        destination_chain -> Int4,
        recipient_address -> Bytea,
        token_id -> Int4,
        amount -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    progress_store,
    iota_error_transactions,
    iota_progress_store,
    token_transfer,
    token_transfer_data,
);
