use beacon_node::{get_config, ClientConfig, ClientGenesis, ProductionBeaconNode};
use clap::ArgMatches;
use environment::{EnvironmentBuilder, LoggerConfig};
use eth2_network_config::{Eth2NetworkConfig, DEFAULT_HARDCODED_NETWORK};
use store::MainnetEthSpec;
use task_executor::ShutdownReason;

pub struct Indexer {}

impl Indexer {
    pub fn start() -> Result<(), String> {
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
        let eth2_network_config = Eth2NetworkConfig::constant(DEFAULT_HARDCODED_NETWORK)?;
        let mut environment = environment_builder
            .optional_eth2_network_config(eth2_network_config)?
            .initialize_logger(logger_config)?
            .multi_threaded_tokio_runtime()?
            .build()?;
        let context = environment.core_context();
        let executor = context.executor.clone();

        let client_config = get_config::<MainnetEthSpec>(&ArgMatches::default(), &context)?;

        executor.clone().spawn(
            async move {
                if let Err(e) = ProductionBeaconNode::new(context.clone(), client_config).await {
                    // Ignore the error since it always occurs during normal operation when
                    // shutting down.
                    let _ = executor
                        .shutdown_sender()
                        .try_send(ShutdownReason::Failure("Failed to start beacon node"));
                }
            },
            "beacon_node",
        );

        let shutdown_reason = environment.block_until_shutdown_requested()?;

        Ok(())
    }
}
