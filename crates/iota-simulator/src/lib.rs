// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(msim)]
use std::hash::Hasher;
use std::sync::atomic::{AtomicUsize, Ordering};

// Re-export things used by iota-macros
pub use ::rand as rand_crate;
pub use anemo;
pub use anemo_tower;
pub use fastcrypto;
pub use iota_framework;
pub use iota_move_build;
pub use iota_types;
pub use lru;
pub use move_package;
#[cfg(msim)]
pub use msim::*;
pub use narwhal_network;
pub use telemetry_subscribers;
pub use tempfile;
pub use tower;

#[cfg(msim)]
pub mod configs {
    use std::{collections::HashMap, ops::Range, time::Duration};

    use msim::*;
    use tracing::info;

    fn ms_to_dur(range: Range<u64>) -> Range<Duration> {
        Duration::from_millis(range.start)..Duration::from_millis(range.end)
    }

    /// A network with constant uniform latency.
    pub fn constant_latency_ms(latency: u64) -> SimConfig {
        uniform_latency_ms(latency..(latency + 1))
    }

    /// A network with latency sampled uniformly from a range.
    pub fn uniform_latency_ms(range: Range<u64>) -> SimConfig {
        let range = ms_to_dur(range);
        SimConfig {
            net: NetworkConfig {
                latency: LatencyConfig {
                    default_latency: LatencyDistribution::uniform(range),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }

    /// A network with bimodal latency.
    pub fn bimodal_latency_ms(
        // The typical latency.
        baseline: Range<u64>,
        // The exceptional latency.
        degraded: Range<u64>,
        // The frequency (from 0.0 to 1.0) with which the exceptional distribution is sampled.
        degraded_freq: f64,
    ) -> SimConfig {
        let baseline = ms_to_dur(baseline);
        let degraded = ms_to_dur(degraded);
        SimConfig {
            net: NetworkConfig {
                latency: LatencyConfig {
                    default_latency: LatencyDistribution::bimodal(
                        baseline,
                        degraded,
                        degraded_freq,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }

    /// Select from among a number of configs using the IOTA_SIM_CONFIG env var.
    pub fn env_config(
        // Config to use when IOTA_SIM_CONFIG is not set.
        default: SimConfig,
        // List of (&str, SimConfig) pairs - the SimConfig associated with the value
        // of the IOTA_SIM_CONFIG var is chosen.
        env_configs: impl IntoIterator<Item = (&'static str, SimConfig)>,
    ) -> SimConfig {
        let mut env_configs = HashMap::<&'static str, SimConfig>::from_iter(env_configs);
        if let Some(env) = std::env::var("IOTA_SIM_CONFIG").ok() {
            if let Some(cfg) = env_configs.remove(env.as_str()) {
                info!("Using test config for IOTA_SIM_CONFIG={}", env);
                cfg
            } else {
                panic!(
                    "No config found for IOTA_SIM_CONFIG={}. Available configs are: {:?}",
                    env,
                    env_configs.keys()
                );
            }
        } else {
            info!("Using default test config");
            default
        }
    }
}

thread_local! {
    static NODE_COUNT: AtomicUsize = const { AtomicUsize::new(0) };
}

pub struct NodeLeakDetector(());

impl NodeLeakDetector {
    pub fn new() -> Self {
        NODE_COUNT.with(|c| c.fetch_add(1, Ordering::SeqCst));
        Self(())
    }

    pub fn get_current_node_count() -> usize {
        NODE_COUNT.with(|c| c.load(Ordering::SeqCst))
    }
}

impl Default for NodeLeakDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for NodeLeakDetector {
    fn drop(&mut self) {
        NODE_COUNT.with(|c| c.fetch_sub(1, Ordering::SeqCst));
    }
}

#[cfg(not(msim))]
#[macro_export]
macro_rules! return_if_killed {
    () => {};
}

#[cfg(msim)]
pub fn current_simnode_id() -> msim::task::NodeId {
    msim::runtime::NodeHandle::current().id()
}

#[cfg(msim)]
pub mod random {
    use std::{cell::RefCell, collections::HashSet, hash::Hash};

    use rand_crate::{rngs::SmallRng, thread_rng, Rng, SeedableRng};
    use serde::Serialize;

    use super::*;

    /// Given a value, produce a random probability using the value as a seed,
    /// with an additional seed that is constant only for the current test
    /// thread.
    pub fn deterministic_probability<T: Hash>(value: T, chance: f32) -> bool {
        thread_local! {
            // a random seed that is shared by the whole test process, so that equal `value`
            // inputs produce different outputs when the test seed changes
            static SEED: u64 = thread_rng().gen();
        }

        chance
            > SEED.with(|seed| {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                seed.hash(&mut hasher);
                value.hash(&mut hasher);
                let mut rng = SmallRng::seed_from_u64(hasher.finish());
                rng.gen_range(0.0..1.0)
            })
    }

    /// Like deterministic_probability, but only returns true once for each
    /// unique value. May eventually consume all memory if there are a large
    /// number of unique, failing values.
    pub fn deterministic_probability_once<T: Hash + Serialize>(value: T, chance: f32) -> bool {
        thread_local! {
            static FAILING_VALUES: RefCell<HashSet<(msim::task::NodeId, Vec<u8>)>> = RefCell::new(HashSet::new());
        }

        let bytes = bcs::to_bytes(&value).unwrap();
        let key = (current_simnode_id(), bytes);

        FAILING_VALUES.with(|failing_values| {
            let mut failing_values = failing_values.borrow_mut();
            if failing_values.contains(&key) {
                false
            } else if deterministic_probability(value, chance) {
                failing_values.insert(key);
                true
            } else {
                false
            }
        })
    }
}

/// TODO: This is a temporary fix to a problem in `msim_macros`, where it
/// generates code like this:
///
/// ```rust
/// let (stop_tx, stop_rx) = ::tokio::sync::oneshot::channel();
/// let watchdog =
///     iota_simulator::runtime::start_watchdog(rt.clone(), inner_seed, watchdog_timeout, stop_rx);
/// ```
///
/// Unfortunately, this uses the local `tokio` of the caller, rather than the
/// `real_tokio` crate in the `msim` workspace that is expected by
/// `start_watchdog`. Replacing the local tokio in each crate where sim tests
/// are used is not a good option, as it conflicts with other crates' usage of
/// tokio. This solution resolves the problem by shadowing the method and
/// converting the channel type with an additional adapter task.
#[cfg(msim)]
pub mod runtime {
    use std::{
        sync::{Arc, RwLock},
        time::Duration,
    };

    pub use msim::runtime::*;

    /// This is an adapter fn to convert the mismatched oneshot types used by
    /// msim
    pub fn start_watchdog(
        rt: Arc<RwLock<Option<Runtime>>>,
        inner_seed: u64,
        timeout: Duration,
        stop: tokio::sync::oneshot::Receiver<()>,
    ) -> std::thread::JoinHandle<()> {
        let (sender, receiver) = real_tokio::sync::oneshot::channel();
        tokio::spawn(async {
            sender.send(stop.await.unwrap()).unwrap();
        });
        msim::runtime::start_watchdog(rt, inner_seed, timeout, receiver)
    }
}
