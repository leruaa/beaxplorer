use std::sync::Arc;

use environment::{Environment, EnvironmentBuilder, LoggerConfig};
use eth2_network_config::{Eth2NetworkConfig, DEFAULT_HARDCODED_NETWORK};
use lighthouse_types::{EthSpec, MainnetEthSpec};
use parking_lot::RwLock;

use slog::warn;
use tokio::{
    signal,
    sync::{
        mpsc::{self, unbounded_channel},
        watch,
    },
};
use types::{
    attestation::AttestationModel,
    block::{BlockExtendedModel, BlockModel},
    block_request::{BlockRequestModel, BlockRequestModelWithId, PersistIteratorBlockRequestModel},
    committee::CommitteeModel,
    epoch::{EpochExtendedModel, EpochModel, EpochModelWithId, PersistIteratorEpochModel},
    good_peer::{GoodPeerModel, GoodPeerModelWithId, PersistIteratorGoodPeerModel},
    path::ToPath,
    validator::ValidatorModel,
    vote::VoteModel,
};

use crate::{
    beacon_chain::beacon_context::BeaconContext,
    direct_indexer::Indexer,
    network::{augmented_network_service::AugmentedNetworkService, peer_db::PeerDb},
};

pub fn start_indexer(reset: bool, base_dir: String) -> Result<(), String> {
    let (environment, _) = build_environment(EnvironmentBuilder::mainnet())?;
    let context = environment.core_context();
    let beacon_context = BeaconContext::build(&context)?;
    let executor = context.executor;
    let log = executor.log().clone();

    if reset {
        if let Err(err) = remove_dirs(&base_dir) {
            warn!(log, "{err}");
        }
    }

    create_dirs(&base_dir)?;

    environment.runtime().block_on(async move {
        let (shutdown_handle, mut shutdown_complete) = mpsc::channel(1);
        let (shutdown_request, shutdown_trigger) = watch::channel(());
        let indexer = Indexer::new(log);

        indexer.spawn_services(
            base_dir,
            executor,
            Arc::new(beacon_context),
            shutdown_handle,
            shutdown_trigger,
        );

        wait_shutdown_signal().await;

        let _ = shutdown_request.send(());

        // When every sender has gone out of scope, the recv call
        // will return with an error. We ignore the error.
        let _ = shutdown_complete.recv().await;
    });

    Ok(())
}

pub fn update_indexes(base_dir: String) -> Result<(), String> {
    EpochModelWithId::iter(&base_dir)?.persist_sortables(&base_dir)?;
    BlockRequestModelWithId::iter(&base_dir)?.persist_sortables(&base_dir)?;
    GoodPeerModelWithId::iter(&base_dir)?.persist_sortables(&base_dir)?;

    Ok(())
}

fn build_environment<E: EthSpec>(
    environment_builder: EnvironmentBuilder<E>,
) -> Result<(Environment<E>, Eth2NetworkConfig), String> {
    let logger_config = LoggerConfig {
        path: None,
        debug_level: String::from("info"),
        logfile_debug_level: String::from("info"),
        log_format: None,
        log_color: true,
        disable_log_timestamp: true,
        max_log_size: 0,
        max_log_number: 0,
        compression: false,
        is_restricted: false,
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

fn create_dirs(base_dir: &str) -> Result<(), String> {
    EpochModel::create_dirs(base_dir)?;
    EpochExtendedModel::create_dirs(base_dir)?;
    BlockModel::create_dirs(base_dir)?;
    BlockExtendedModel::create_dirs(base_dir)?;
    AttestationModel::create_dirs(base_dir)?;
    CommitteeModel::create_dirs(base_dir)?;
    VoteModel::create_dirs(base_dir)?;
    ValidatorModel::create_dirs(base_dir)?;
    BlockRequestModel::create_dirs(base_dir)?;
    GoodPeerModel::create_dirs(base_dir)?;

    Ok(())
}

fn remove_dirs(base_dir: &str) -> Result<(), String> {
    EpochModel::remove_dirs(base_dir)?;
    EpochExtendedModel::remove_dirs(base_dir)?;
    BlockModel::remove_dirs(base_dir)?;
    BlockExtendedModel::remove_dirs(base_dir)?;
    AttestationModel::remove_dirs(base_dir)?;
    CommitteeModel::remove_dirs(base_dir)?;
    VoteModel::remove_dirs(base_dir)?;
    ValidatorModel::remove_dirs(base_dir)?;
    BlockRequestModel::remove_dirs(base_dir)?;

    Ok(())
}

async fn wait_shutdown_signal() {
    match signal::ctrl_c().await {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
        }
    }
}
