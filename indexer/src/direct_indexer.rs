use std::{env, pin::Pin};

use beacon_node::get_config;
use clap::ArgMatches;
use environment::{Environment, EnvironmentBuilder, LoggerConfig};
use eth2_network_config::{Eth2NetworkConfig, DEFAULT_HARDCODED_NETWORK};
use futures::Future;
use libp2p::{Multiaddr, PeerId};
use lighthouse_network::{
    rpc::{BlocksByRangeRequest, BlocksByRootRequest},
    Request,
};
use store::{beacon_block, Hash256, MainnetEthSpec};

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
        let beacon_context = BeaconContext::build(&context)?;
        let executor = context.executor.clone();

        executor.clone().spawn(
            async move {
                let root: Hash256 =
                    "0x70ffb2f48d9dc3ba835ebd0a4bd34e2d7b09bc6d4ef3b46c74131b6cbf952a90"
                        .parse()
                        .unwrap();

                let log = executor.clone().log().clone();
                let network_service = NetworkService::new(executor, beacon_context).await.unwrap();
                let mut request_manager = RequestManager::new(network_service, log);

                request_manager.send_request(Request::BlocksByRange(BlocksByRangeRequest {
                    start_slot: 0,
                    count: 32,
                    step: 1,
                }));
                /*
                request_manager.send_request(Request::BlocksByRoot(BlocksByRootRequest {
                    block_roots: vec![root].into(),
                }));
                 */

                loop {
                    let response = request_manager.next_event().await;
                    println!("{:?}", response);
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
            debug_level: "info",
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
