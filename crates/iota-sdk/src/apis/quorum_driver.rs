// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::time::{Duration, Instant};

use iota_json_rpc_api::{ReadApiClient, WriteApiClient};
use iota_json_rpc_types::{IotaTransactionBlockResponse, IotaTransactionBlockResponseOptions};
use iota_types::{quorum_driver_types::ExecuteTransactionRequestType, transaction::Transaction};

use crate::{
    SharedRpcClient,
    error::{Error, IotaRpcResult},
};

const WAIT_FOR_LOCAL_EXECUTION_TIMEOUT: Duration = Duration::from_secs(60);
const WAIT_FOR_LOCAL_EXECUTION_DELAY: Duration = Duration::from_millis(200);
const WAIT_FOR_LOCAL_EXECUTION_INTERVAL: Duration = Duration::from_secs(2);

/// Quorum API that provides functionality to execute a transaction block and
/// submit it to the fullnode(s).
#[derive(Clone, Debug)]
pub struct QuorumDriverApi {
    api: SharedRpcClient,
}

impl QuorumDriverApi {
    pub(crate) fn new(api: SharedRpcClient) -> Self {
        Self { api }
    }

    /// Execute a transaction with a FullNode client. `request_type`
    /// defaults to `ExecuteTransactionRequestType::WaitForLocalExecution`.
    /// When `ExecuteTransactionRequestType::WaitForLocalExecution` is used,
    /// but returned `confirmed_local_execution` is false, the client will
    /// keep retry for WAIT_FOR_LOCAL_EXECUTION_RETRY_COUNT times. If it
    /// still fails, it will return an error.
    pub async fn execute_transaction_block(
        &self,
        tx: Transaction,
        options: IotaTransactionBlockResponseOptions,
        request_type: impl Into<Option<ExecuteTransactionRequestType>>,
    ) -> IotaRpcResult<IotaTransactionBlockResponse> {
        let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();
        let request_type = request_type
            .into()
            .unwrap_or_else(|| options.default_execution_request_type());

        let start = Instant::now();
        let response = self
            .api
            .http
            .execute_transaction_block(
                tx_bytes.clone(),
                signatures.clone(),
                Some(options.clone()),
                Some(request_type.clone()),
            )
            .await?;

        if let ExecuteTransactionRequestType::WaitForEffectsCert = request_type {
            return Ok(response);
        }

        // JSON-RPC ignores WaitForLocalExecution, so simulate it by polling for the
        // transaction.
        let mut poll_response = tokio::time::timeout(WAIT_FOR_LOCAL_EXECUTION_TIMEOUT, async {
            // Apply a short delay to give the full node a chance to catch up.
            tokio::time::sleep(WAIT_FOR_LOCAL_EXECUTION_DELAY).await;

            let mut interval = tokio::time::interval(WAIT_FOR_LOCAL_EXECUTION_INTERVAL);
            loop {
                interval.tick().await;

                if let Ok(poll_response) = self
                    .api
                    .http
                    .get_transaction_block(*tx.digest(), Some(options.clone()))
                    .await
                {
                    break poll_response;
                }
            }
        })
        .await
        .map_err(|_| {
            Error::FailToConfirmTransactionStatus(*tx.digest(), start.elapsed().as_secs())
        })?;

        poll_response.confirmed_local_execution = Some(true);
        Ok(poll_response)
    }
}
