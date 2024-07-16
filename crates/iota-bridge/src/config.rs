// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
    collections::{BTreeMap, HashSet},
    path::PathBuf,
    str::FromStr,
    sync::Arc,
};

use alloy::{primitives::Address as EthAddress, providers::RootProvider, transports::http::Http};
use anyhow::anyhow;
use fastcrypto::traits::EncodeDecodeBase64;
use iota_config::Config;
use iota_sdk::IotaClient as IotaSdkClient;
use iota_types::{
    base_types::{IotaAddress, ObjectID, ObjectRef},
    crypto::IotaKeyPair,
    event::EventID,
    object::Owner,
    Identifier,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use tracing::info;

use crate::{
    crypto::BridgeAuthorityKeyPair, error::BridgeError, eth_client::EthClient,
    iota_client::IotaClient, types::BridgeAction,
};

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct BridgeNodeConfig {
    /// The port that the server listens on.
    pub server_listen_port: u16,
    /// The port that for metrics server.
    pub metrics_port: u16,
    /// Path of the file where bridge authority key (Secp256k1) is stored as
    /// Base64 encoded `privkey`.
    pub bridge_authority_key_path_base64_raw: PathBuf,
    /// Rpc url for Iota fullnode, used for query stuff and submit transactions.
    pub iota_rpc_url: String,
    /// Rpc url for Eth fullnode, used for query stuff.
    pub eth_rpc_url: String,
    /// The eth contract addresses (hex). It must not be empty. It serves two
    /// purpose:
    /// 1. validator only signs bridge actions that are generated from these
    ///    contracts.
    /// 2. for EthSyncer to watch for when `run_client` is true.
    pub eth_addresses: Vec<String>,
    /// Path of the file where bridge client key (any IotaKeyPair) is stored as
    /// Base64 encoded `flag || privkey`. If `run_client` is true, and this
    /// is None, then use `bridge_authority_key_path_base64_raw` as client key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_client_key_path_base64_iota_key: Option<PathBuf>,
    /// Whether to run client. If true,
    /// `bridge_client_key_path_base64_iota_key`, `bridge_client_gas_object`
    /// and `db_path` needs to be provided.
    pub run_client: bool,
    /// The gas object to use for paying for gas fees for the client. It needs
    /// to be owned by the address associated with bridge client key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_client_gas_object: Option<ObjectID>,
    /// Path of the client storage. Required when `run_client` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_path: Option<PathBuf>,
    // TODO: this should be hardcoded and removed from config
    /// The iota modules of bridge packages for client to watch for. Need to
    /// contain at least one item when `run_client` is true.
    pub iota_bridge_modules: Option<Vec<String>>,
    // TODO: we need to hardcode the starting blocks for eth networks for cold start.
    /// Override the start block number for each eth address. Key must be in
    /// `eth_addresses`. When set, EthSyncer will start from this block
    /// number (inclusively) instead of the one in storage.
    /// Key: eth address, Value:  block number to start from
    /// Note: This field should be rarely used. Only use it when you understand
    /// how to follow up.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eth_bridge_contracts_start_block_override: Option<BTreeMap<String, u64>>,
    /// Override the last processed EventID for each bridge module. Key must be
    /// in `iota_bridge_modules`. When set, IotaSyncer will start from this
    /// cursor (exclusively) instead of the one in storage. Key: iota module,
    /// Value: last processed EventID (tx_digest, event_seq). Note 1: This
    /// field should be rarely used. Only use it when you understand how to
    /// follow up. Note 2: the EventID needs to be valid, namely it must
    /// exist and matches the filter. Otherwise, it will miss one event
    /// because of fullnode Event query semantics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iota_bridge_modules_last_processed_event_id_override: Option<BTreeMap<String, EventID>>,
    /// A list of approved governance actions. Action in this list will be
    /// signed when requested by client.
    pub approved_governance_actions: Vec<BridgeAction>,
}

impl Config for BridgeNodeConfig {}

impl BridgeNodeConfig {
    pub async fn validate(
        &self,
    ) -> anyhow::Result<(BridgeServerConfig, Option<BridgeClientConfig>)> {
        let bridge_authority_key =
            read_bridge_authority_key(&self.bridge_authority_key_path_base64_raw)?;

        // TODO: verify it's part of bridge committee
        let iota_client = Arc::new(IotaClient::<IotaSdkClient>::new(&self.iota_rpc_url).await?);

        // TODO(audit-blocking): verify Iota Chain ID matches bridge Chain ID

        if self.eth_addresses.is_empty() {
            return Err(anyhow!("`eth_addresses` must contain at least one address"));
        }
        let eth_bridge_contracts = self
            .eth_addresses
            .iter()
            .map(|addr| EthAddress::from_str(addr))
            .collect::<Result<Vec<_>, _>>()?;
        let eth_client = Arc::new(
            EthClient::<RootProvider<Http<Client>>>::new(
                &self.eth_rpc_url,
                HashSet::from_iter(eth_bridge_contracts.iter().cloned()),
            )
            .await?,
        );
        // TODO(audit-blocking): verify Ethereum Chain ID matches bridge Chain ID

        // Validate approved actions that must be governace actions
        for action in &self.approved_governance_actions {
            if !action.is_governace_action() {
                return Err(anyhow::anyhow!(format!(
                    "{:?}",
                    BridgeError::ActionIsNotGovernanceAction(action.clone())
                )));
            }
        }
        let approved_governance_actions = self.approved_governance_actions.clone();

        let bridge_server_config = BridgeServerConfig {
            key: bridge_authority_key,
            metrics_port: self.metrics_port,
            server_listen_port: self.server_listen_port,
            iota_client: iota_client.clone(),
            eth_client: eth_client.clone(),
            approved_governance_actions,
        };

        if !self.run_client {
            return Ok((bridge_server_config, None));
        }
        // If client is enabled, prepare client config
        let bridge_client_key = if self.bridge_client_key_path_base64_iota_key.is_none() {
            let bridge_client_key =
                read_bridge_authority_key(&self.bridge_authority_key_path_base64_raw)?;
            Ok(IotaKeyPair::from(bridge_client_key))
        } else {
            read_bridge_client_key(
                self.bridge_client_key_path_base64_iota_key
                    .as_ref()
                    .unwrap(),
            )
        }?;

        let client_iota_address = IotaAddress::from(&bridge_client_key.public());
        info!("Bridge client iota address: {:?}", client_iota_address);
        let gas_object_id = self.bridge_client_gas_object.ok_or(anyhow!(
            "`bridge_client_gas_object` is required when `run_client` is true"
        ))?;
        let db_path = self
            .db_path
            .clone()
            .ok_or(anyhow!("`db_path` is required when `run_client` is true"))?;

        let mut eth_bridge_contracts_start_block_override = BTreeMap::new();
        match &self.eth_bridge_contracts_start_block_override {
            Some(overrides) => {
                for (addr, block_number) in overrides {
                    let address = EthAddress::from_str(addr)?;
                    if eth_bridge_contracts.contains(&address) {
                        eth_bridge_contracts_start_block_override.insert(address, *block_number);
                    } else {
                        return Err(anyhow!(
                            "Override start block number for address {:?} is not in `eth_addresses`",
                            addr
                        ));
                    }
                }
            }
            None => {}
        }

        let iota_bridge_modules = match &self.iota_bridge_modules {
            Some(modules) => {
                if modules.is_empty() {
                    return Err(anyhow!(
                        "`iota_bridge_modules` is required when `run_client` is true"
                    ));
                }
                modules
                    .iter()
                    .map(|module| Identifier::from_str(module))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| anyhow!("Error parsing iota module: {:?}", e))?
            }
            None => {
                return Err(anyhow!(
                    "`iota_bridge_modules` is required when `run_client` is true"
                ));
            }
        };

        let mut iota_bridge_modules_last_processed_event_id_override = BTreeMap::new();
        match &self.iota_bridge_modules_last_processed_event_id_override {
            Some(overrides) => {
                for (module, cursor) in overrides {
                    let module = Identifier::from_str(module)?;
                    if iota_bridge_modules.contains(&module) {
                        iota_bridge_modules_last_processed_event_id_override
                            .insert(module, *cursor);
                    } else {
                        return Err(anyhow!(
                            "Override start tx digest for module {:?} is not in `iota_bridge_modules`",
                            module
                        ));
                    }
                }
            }
            None => {}
        }

        let (gas_coin, gas_object_ref, owner) = iota_client
            .get_gas_data_panic_if_not_gas(gas_object_id)
            .await;
        if owner != Owner::AddressOwner(client_iota_address) {
            return Err(anyhow!(
                "Gas object {:?} is not owned by bridge client key's associated iota address {:?}, but {:?}",
                gas_object_id,
                client_iota_address,
                owner
            ));
        }
        info!(
            "Starting bridge client with gas object {:?}, balance: {}",
            gas_object_ref.0,
            gas_coin.value()
        );
        let bridge_client_config = BridgeClientConfig {
            iota_address: client_iota_address,
            key: bridge_client_key,
            gas_object_ref,
            metrics_port: self.metrics_port,
            iota_client: iota_client.clone(),
            eth_client: eth_client.clone(),
            db_path,
            eth_bridge_contracts,
            iota_bridge_modules,
            eth_bridge_contracts_start_block_override,
            iota_bridge_modules_last_processed_event_id_override,
        };

        Ok((bridge_server_config, Some(bridge_client_config)))
    }
}

pub struct BridgeServerConfig {
    pub key: BridgeAuthorityKeyPair,
    pub server_listen_port: u16,
    pub metrics_port: u16,
    pub iota_client: Arc<IotaClient<IotaSdkClient>>,
    pub eth_client: Arc<EthClient<RootProvider<Http<Client>>>>,
    /// A list of approved governance actions. Action in this list will be
    /// signed when requested by client.
    pub approved_governance_actions: Vec<BridgeAction>,
}

// TODO: add gas balance alert threshold
pub struct BridgeClientConfig {
    pub iota_address: IotaAddress,
    pub key: IotaKeyPair,
    pub gas_object_ref: ObjectRef,
    pub metrics_port: u16,
    pub iota_client: Arc<IotaClient<IotaSdkClient>>,
    pub eth_client: Arc<EthClient<RootProvider<Http<Client>>>>,
    pub db_path: PathBuf,
    pub eth_bridge_contracts: Vec<EthAddress>,
    pub iota_bridge_modules: Vec<Identifier>,
    pub eth_bridge_contracts_start_block_override: BTreeMap<EthAddress, u64>,
    pub iota_bridge_modules_last_processed_event_id_override: BTreeMap<Identifier, EventID>,
}

/// Read Bridge Authority key (Secp256k1KeyPair) from a file.
/// BridgeAuthority key is stored as base64 encoded `privkey`.
pub fn read_bridge_authority_key(path: &PathBuf) -> Result<BridgeAuthorityKeyPair, anyhow::Error> {
    if !path.exists() {
        return Err(anyhow::anyhow!(
            "Bridge authority key file not found at path: {:?}",
            path
        ));
    }
    let contents = std::fs::read_to_string(path)?;

    BridgeAuthorityKeyPair::decode_base64(contents.as_str().trim())
        .map_err(|e| anyhow!("Error decoding authority key: {:?}", e))
}

/// Read Bridge client key (any IotaKeyPair) from a file.
/// Read from file as Base64 encoded `flag || privkey`.
pub fn read_bridge_client_key(path: &PathBuf) -> Result<IotaKeyPair, anyhow::Error> {
    if !path.exists() {
        return Err(anyhow::anyhow!(
            "Bridge client key file not found at path: {:?}",
            path
        ));
    }
    let contents = std::fs::read_to_string(path)?;

    IotaKeyPair::decode_base64(contents.as_str().trim())
        .map_err(|e| anyhow!("Error decoding authority key: {:?}", e))
}
