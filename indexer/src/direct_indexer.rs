use std::{borrow::BorrowMut, env, pin::Pin};

use environment::{Environment, EnvironmentBuilder, LoggerConfig};
use eth2_network_config::{Eth2NetworkConfig, DEFAULT_HARDCODED_NETWORK};
use futures::Future;
use lighthouse_network::{rpc::BlocksByRangeRequest, Request};
use slog::info;
use store::{Hash256, MainnetEthSpec};

use state_processing::{
    per_block_processing, per_slot_processing, BlockSignatureStrategy, VerifyBlockRoot,
};

use crate::{
    beacon_chain::beacon_context::BeaconContext,
    beacon_node_client::BeaconNodeClient,
    network::{network_service::NetworkService, request_manager::RequestManager},
};

// use the executor for libp2p
struct Executor(task_executor::TaskExecutor);

impl libp2p::core::Executor for Executor {
    fn exec(&self, f: Pin<Box<dyn Future<Output = ()> + Send>>) {
        self.0.spawn(f, "libp2p");
    }
}

pub struct Indexer;

impl Indexer {
    pub fn start() -> Result<(), String> {
        let endpoint = env::var("ENDPOINT_URL").unwrap();
        let client = BeaconNodeClient::new(endpoint);
        let (mut environment, eth2_network_config) = Self::build_environment().unwrap();
        let context = environment.core_context();
        let mut beacon_context = BeaconContext::build(&context)?;
        let executor = context.executor;

        executor.clone().spawn(
            async move {
                let root: Hash256 =
                    "0x70ffb2f48d9dc3ba835ebd0a4bd34e2d7b09bc6d4ef3b46c74131b6cbf952a90"
                        .parse()
                        .unwrap();

                let log = executor.clone().log().clone();
                let network_service = NetworkService::new(executor, &beacon_context)
                    .await
                    .unwrap();
                let mut request_manager = RequestManager::new(network_service, log.clone());

                request_manager.send_request(Request::BlocksByRange(BlocksByRangeRequest {
                    start_slot: 1,
                    count: 31,
                    step: 1,
                }));

                loop {
                    let current_state = beacon_context.genesis_state.borrow_mut();

                    match request_manager.next_event().await {
                        lighthouse_network::Response::BlocksByRange(block) => match block {
                            Some(block) => {
                                per_slot_processing(current_state, None, &beacon_context.spec)
                                    .unwrap();
                                info!(log, "State: {:?}", current_state.slot());
                                info!(log, "Adding: {:?}", block.slot());

                                per_block_processing::<MainnetEthSpec>(
                                    current_state,
                                    &block,
                                    None,
                                    BlockSignatureStrategy::NoVerification,
                                    VerifyBlockRoot::True,
                                    &beacon_context.spec,
                                )
                                .unwrap();
                                info!(log, "New block added: {:?}", current_state.slot());
                            }
                            None => todo!(),
                        },
                        lighthouse_network::Response::BlocksByRoot(_) => todo!(),
                        _ => {}
                    }
                }
            },
            "network",
        );

        environment.block_until_shutdown_requested().unwrap();

        Ok(())
    }

    fn build_environment() -> Result<(Environment<MainnetEthSpec>, Eth2NetworkConfig), String> {
        let environment_builder = EnvironmentBuilder::mainnet();
        let logger_config = LoggerConfig {
            path: None,
            debug_level: "debug",
            logfile_debug_level: "info",
            log_format: None,
            max_log_size: 0,
            max_log_number: 0,
            compression: false,
        };
        let eth2_network_config = Eth2NetworkConfig::constant(DEFAULT_HARDCODED_NETWORK)?
            .ok_or("Failed to build Eth2 network config")?;
        let environment = environment_builder
            .eth2_network_config(eth2_network_config.clone())?
            .initialize_logger(logger_config)?
            .multi_threaded_tokio_runtime()?
            .build()?;

        Ok((environment, eth2_network_config))
    }
}
