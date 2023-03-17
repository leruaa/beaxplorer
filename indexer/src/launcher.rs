use std::{fs, sync::Arc};

use environment::{Environment, EnvironmentBuilder, LoggerConfig};
use eth2_network_config::{Eth2NetworkConfig, DEFAULT_HARDCODED_NETWORK};
use lighthouse_types::{EthSpec, MainnetEthSpec};
use parking_lot::RwLock;

use tokio::{
    signal,
    sync::{
        mpsc::{self, unbounded_channel},
        watch,
    },
};
use types::{
    block_request::{BlockRequestModelWithId, PersistIteratorBlockRequestModel},
    epoch::{EpochModelWithId, PersistIteratorEpochModel},
};

use crate::{
    beacon_chain::beacon_context::BeaconContext,
    direct_indexer::Indexer,
    network::{
        augmented_network_service::AugmentedNetworkService,
        block_by_root_requests::BlockByRootRequests,
        peer_db::PeerDb,
        persist_service::{PersistMessage, PersistService},
        workers::block_by_root_requests_worker::BlockByRootRequestsWorker,
    },
};

pub fn start_indexer(reset: bool, base_dir: String) -> Result<(), String> {
    let (environment, _) = build_environment(EnvironmentBuilder::mainnet())?;
    let context = environment.core_context();
    let beacon_context = BeaconContext::build(&context)?;
    let executor = context.executor;
    let log = executor.log().clone();

    if reset {
        fs::remove_dir_all(&base_dir).unwrap();
    }

    fs::create_dir_all(base_dir.clone() + "/epochs/e/").unwrap();
    fs::create_dir_all(base_dir.clone() + "/epochs/s/attestations_count/").unwrap();
    fs::create_dir_all(base_dir.clone() + "/epochs/s/deposits_count/").unwrap();
    fs::create_dir_all(base_dir.clone() + "/epochs/s/attester_slashings_count/").unwrap();
    fs::create_dir_all(base_dir.clone() + "/epochs/s/proposer_slashings_count/").unwrap();
    fs::create_dir_all(base_dir.clone() + "/epochs/s/eligible_ether/").unwrap();
    fs::create_dir_all(base_dir.clone() + "/epochs/s/voted_ether/").unwrap();
    fs::create_dir_all(base_dir.clone() + "/epochs/s/global_participation_rate/").unwrap();
    fs::create_dir_all(base_dir.clone() + "/blocks").unwrap();
    fs::create_dir_all(base_dir.clone() + "/blocks/e/").unwrap();
    fs::create_dir_all(base_dir.clone() + "/blocks/a/").unwrap();
    fs::create_dir_all(base_dir.clone() + "/blocks/c/").unwrap();
    fs::create_dir_all(base_dir.clone() + "/blocks/v/").unwrap();
    fs::create_dir_all(base_dir.clone() + "/block_requests").unwrap();
    fs::create_dir_all(base_dir.clone() + "/block_requests/s/root/").unwrap();
    fs::create_dir_all(base_dir.clone() + "/good_peers").unwrap();
    fs::create_dir_all(base_dir.clone() + "/good_peers/s/id/").unwrap();
    fs::create_dir_all(base_dir.clone() + "/validators").unwrap();

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
    let all_epochs = EpochModelWithId::all(&base_dir)?;
    let all_block_requests = BlockRequestModelWithId::all(&base_dir)?;

    all_epochs.into_iter().persist_sortables(&base_dir)?;

    all_block_requests
        .into_iter()
        .persist_sortables(&base_dir)?;

    Ok(())
}

pub fn search_orphans(base_dir: String) -> Result<(), String> {
    let (mut environment, _) = build_environment(EnvironmentBuilder::mainnet())?;
    let context = environment.core_context();
    let beacon_context = Arc::new(BeaconContext::build(&context)?);
    let executor = context.executor;
    let log = executor.log().clone();

    executor.clone().spawn(
        async move {
            let (network_command_send, mut network_event_recv, network_globals) =
                AugmentedNetworkService::start(executor.clone(), beacon_context.clone())
                    .await
                    .unwrap();

            let (persist_send, persist_recv) =
                unbounded_channel::<PersistMessage<MainnetEthSpec>>();
            let (shutdown_request, shutdown_trigger) = watch::channel(());
            let peer_db = Arc::new(PeerDb::new(network_globals.clone(), log.clone()));
            let block_requests = BlockRequestModelWithId::all(&base_dir).unwrap();
            let block_by_root_requests = BlockByRootRequests::from_block_requests(block_requests);

            let mut block_by_root_requests_worker = BlockByRootRequestsWorker::new(
                peer_db,
                network_command_send.clone(),
                persist_send,
                Arc::new(RwLock::new(block_by_root_requests)),
                log.clone(),
            );

            PersistService::spawn(
                executor,
                base_dir,
                beacon_context,
                persist_recv,
                shutdown_trigger,
                log.clone(),
            );

            while let Some(event) = network_event_recv.recv().await {
                block_by_root_requests_worker.handle_event(&event)
            }
        },
        "discovery",
    );

    environment.block_until_shutdown_requested().unwrap();

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

async fn wait_shutdown_signal() {
    match signal::ctrl_c().await {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
        }
    }
}
