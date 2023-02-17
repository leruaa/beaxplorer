use std::{fs, time::Duration};

use environment::{Environment, EnvironmentBuilder, LoggerConfig};
use eth2_network_config::{Eth2NetworkConfig, DEFAULT_HARDCODED_NETWORK};
use lighthouse_network::{rpc::BlocksByRootRequest, NetworkEvent, Request};
use lighthouse_types::{EthSpec, Hash256, MainnetEthSpec};
use slog::info;
use tokio::{
    sync::mpsc,
    time::{interval_at, Instant},
};

use crate::{
    beacon_chain::beacon_context::BeaconContext,
    direct_indexer::{BlockMessage, Indexer},
    network::{
        augmented_network_service::{AugmentedNetworkService, NetworkMessage, RequestId},
        peer_db::PeerDb,
        worker::Worker,
    },
};

pub fn start_indexer(reset: bool, base_dir: String) -> Result<(), String> {
    let (mut environment, _) = build_environment(EnvironmentBuilder::mainnet())?;
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
    fs::create_dir_all(base_dir.clone() + "/validators").unwrap();

    executor.clone().spawn(
        async move {
            let (network_send, mut behavior_recv, network_globals) =
                AugmentedNetworkService::start(executor.clone(), &beacon_context)
                    .await
                    .unwrap();

            let (block_send, block_recv) =
                mpsc::unbounded_channel::<BlockMessage<MainnetEthSpec>>();

            let indexer = Indexer::new(
                base_dir,
                beacon_context.genesis_state.clone(),
                beacon_context.spec.clone(),
                log.clone(),
            );

            indexer.spawn_notifier(&executor, network_globals.clone());

            indexer.spawn_indexer(&executor, block_recv);

            let mut worker = Worker::new(
                PeerDb::new(network_globals, log.clone()),
                network_send.clone(),
                block_send,
                log.clone(),
            );

            let start_instant = Instant::now();
            let interval_duration = Duration::from_secs(1);
            let mut interval = interval_at(start_instant, interval_duration);

            loop {
                tokio::select! {
                    Some(event) = behavior_recv.recv() => worker.handle_event(event),
                    _ = interval.tick() => worker.notify()
                }
            }
        },
        "indexer",
    );

    environment.block_until_shutdown_requested().unwrap();

    Ok(())
}

pub fn start_discovery() -> Result<(), String> {
    let (mut environment, _) = build_environment(EnvironmentBuilder::mainnet())?;
    let context = environment.core_context();
    let beacon_context = BeaconContext::build(&context)?;
    let executor = context.executor;
    let log = executor.log().clone();

    executor.clone().spawn(
        async move {
            let (network_send, mut behavior_recv, network_globals) =
                AugmentedNetworkService::start(executor.clone(), &beacon_context)
                    .await
                    .unwrap();

            let unknown: Hash256 =
                "0x2f864ae1a78365ae6fcc8d2a52355eeaeb6f4b568ddbb0ff2ffaa1d9406a7fe8"
                    .parse()
                    .unwrap();

            while let Some(event) = behavior_recv.recv().await {
                match event {
                    NetworkEvent::PeerConnectedOutgoing(peer_id) => network_send
                        .send(NetworkMessage::SendRequest {
                            peer_id,
                            request_id: RequestId::Block(unknown),
                            request: Box::new(Request::BlocksByRoot(BlocksByRootRequest {
                                block_roots: vec![unknown].into(),
                            })),
                        })
                        .unwrap(),

                    NetworkEvent::ResponseReceived { peer_id, .. } => {
                        if let Some(i) = network_globals.peers.read().peer_info(&peer_id) {
                            info!(log, "Block found by {peer_id} ({:?})", i.client().kind);
                            for a in i.listening_addresses() {
                                info!(log, "Address: {}", a);
                            }
                        }
                    }

                    _ => {}
                }
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
