use std::{env, pin::Pin, str::FromStr, sync::Arc, time::Duration};

use beacon_node::{
    beacon_chain::{
        builder::BeaconChainBuilder,
        eth1_chain::DummyEth1ChainBackend,
        slot_clock::{SlotClock, SystemTimeSlotClock},
    },
    get_config,
};
use clap::ArgMatches;
use environment::{Environment, EnvironmentBuilder, LoggerConfig};
use eth2_network_config::{Eth2NetworkConfig, DEFAULT_HARDCODED_NETWORK};
use futures::{Future, StreamExt};
use libp2p::{
    swarm::{SwarmBuilder, SwarmEvent},
    Multiaddr,
};
use lighthouse_network::{
    libp2p::{
        bandwidth::BandwidthLogging,
        core::{muxing::StreamMuxerBox, transport::Boxed},
        dns::TokioDnsConfig,
        tcp::TokioTcpConfig,
        Transport,
    },
    peer_manager::Keypair,
    rpc::{BlocksByRootRequest, RequestId, RPC},
    PeerId, PeerIdSerialized, Request,
};
use network::NetworkService;
use store::{BeaconState, ForkContext, Hash256, HotColdDB, LevelDB, MainnetEthSpec};
use task_executor::ShutdownReason;

use crate::{beacon_node_client::BeaconNodeClient, network::service::Service};

type BoxedTransport = Boxed<(PeerId, StreamMuxerBox)>;

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
        let (mut environment, network_config) = Self::build_environment().unwrap();
        let context = environment.core_context();
        let executor = context.executor.clone();

        executor.spawn(
            async move {
                let peers = client
                    .get_connected_peers::<MainnetEthSpec>()
                    .await
                    .unwrap();

                println!("Peers: {:?}", peers);

                let peer_id = "16Uiu2HAkwgkdraX5wvaCkuRi1YdU5VUvpdQH42Un2DXyADYXAD8Q".parse().unwrap();

                let remote: Multiaddr =
                    "/ip4/192.168.1.12/tcp/9000/p2p/16Uiu2HAkwgkdraX5wvaCkuRi1YdU5VUvpdQH42Un2DXyADYXAD8Q"
                        .parse()
                        .unwrap();

                let root: Hash256 =
                    "0x84e1e9d854cd679d06de0ad9c006c0ce93bde7fda6259f4b0a9827666da9cad2"
                        .parse()
                        .unwrap();

                let mut service = Service::new(context.clone(), network_config, peers).unwrap();

                service.connect(remote).unwrap();

                service
                    .send_request(
                        peer_id,
                        Request::BlocksByRoot(BlocksByRootRequest {
                            block_roots: vec![root].into(),
                        }),
                    )
                    .await;

                
                loop {
                    let _ = service.next().await;
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

    pub fn start_p2p() -> Result<(), String> {
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
            .optional_eth2_network_config(eth2_network_config.clone())?
            .initialize_logger(logger_config)?
            .multi_threaded_tokio_runtime()?
            .build()?;
        let context = environment.core_context();
        let spec = context.eth2_config().spec.clone();
        let genesis_state_bytes = eth2_network_config.unwrap().genesis_state_bytes.unwrap();
        let genesis_state =
            BeaconState::<MainnetEthSpec>::from_ssz_bytes(&genesis_state_bytes, &spec)
                .map_err(|e| format!("Unable to parse genesis state SSZ: {:?}", e))?;
        let fork_context = Arc::new(ForkContext::new::<MainnetEthSpec>(
            spec.genesis_slot,
            genesis_state.genesis_validators_root(),
            &spec,
        ));

        context.clone().executor.spawn(
            async move {
                let my_peer_id =
                PeerIdSerialized::from_str("16Uiu2HAkwgkdraX5wvaCkuRi1YdU5VUvpdQH42Un2DXyADYXAD8Q")
                    .unwrap();
                let local_key = Keypair::generate_ed25519();
                let local_peer_id = PeerId::from(local_key.public());
                let transport = Self::build_transport(local_key).unwrap();
                let behaviour = RPC::<MainnetEthSpec>::new(fork_context, context.executor.log().clone());
                let mut swarm = SwarmBuilder::new(transport, behaviour, local_peer_id)
                    .executor(Box::new(Executor(context.clone().executor)))
                    .build();

                swarm
                    .listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
                    .unwrap();

                let remote: Multiaddr =
                    "/ip4/192.168.1.12/tcp/9000/p2p/16Uiu2HAkwgkdraX5wvaCkuRi1YdU5VUvpdQH42Un2DXyADYXAD8Q"
                        .parse()
                        .unwrap();

                let root: Hash256 = "0x70ffb2f48d9dc3ba835ebd0a4bd34e2d7b09bc6d4ef3b46c74131b6cbf952a90".parse().unwrap();
                //let root: Hash256 = "0x84e1e9d854cd679d06de0ad9c006c0ce93bde7fda6259f4b0a9827666da9cad2".parse().unwrap();

                swarm.dial(remote).unwrap();

                loop {
                    match swarm.select_next_some().await {
                        SwarmEvent::NewListenAddr { address, .. } => {
                            println!("Listening on {:?}", address)
                        }
                        SwarmEvent::Behaviour(e) => println!("Behaviour: {:?}", e.event),
                        SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                            println!("Connected to {:?}", peer_id);

                            swarm.behaviour_mut().send_request(
                                my_peer_id.clone().into(),
                                RequestId::Behaviour,
                                Request::BlocksByRoot(BlocksByRootRequest {
                                    block_roots: vec![root].into()
                                })
                                .into(),
                            );

                        }
                        SwarmEvent::OutgoingConnectionError { error, .. } => {
                            println!("Error {:?}", error)
                        }
                        _ => println!("Something happened..."),
                    }
                }
            },
            "light client",
        );

        environment.block_until_shutdown_requested()?;

        Ok(())
    }

    fn build_transport(local_private_key: Keypair) -> Result<BoxedTransport, String> {
        let tcp = TokioTcpConfig::new().nodelay(true);
        let transport = TokioDnsConfig::system(tcp).map_err(|err| err.to_string());

        let (transport, _) = BandwidthLogging::new(transport.unwrap());

        // mplex config
        let mut mplex_config = libp2p::mplex::MplexConfig::new();
        mplex_config.set_max_buffer_size(256);
        mplex_config.set_max_buffer_behaviour(libp2p::mplex::MaxBufferBehaviour::Block);

        // yamux config
        let mut yamux_config = libp2p::yamux::YamuxConfig::default();
        yamux_config.set_window_update_mode(libp2p::yamux::WindowUpdateMode::on_read());

        let transport = transport
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(Self::generate_noise_config(&local_private_key))
            .multiplex(libp2p::core::upgrade::SelectUpgrade::new(
                yamux_config,
                mplex_config,
            ))
            .timeout(Duration::from_secs(10))
            .boxed();

        Ok(transport)
    }

    fn generate_noise_config(
        identity_keypair: &Keypair,
    ) -> libp2p::noise::NoiseAuthenticated<libp2p::noise::XX, libp2p::noise::X25519Spec, ()> {
        let static_dh_keys = libp2p::noise::Keypair::<libp2p::noise::X25519Spec>::new()
            .into_authentic(identity_keypair)
            .expect("signing can fail only once during starting a node");
        libp2p::noise::NoiseConfig::xx(static_dh_keys).into_authenticated()
    }

    /*
    pub fn start_network() -> Result<(), String> {
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
            .optional_eth2_network_config(eth2_network_config.clone())?
            .initialize_logger(logger_config)?
            .multi_threaded_tokio_runtime()?
            .build()?;
        let context = environment.core_context();
        let executor = context.executor.clone();
        let client_config = get_config::<MainnetEthSpec>(&ArgMatches::default(), &context)?;
        let mut network_config = client_config.network;
        let network_log = executor.log().clone();
        let spec = context.eth2_config().spec.clone();
        let genesis_state_bytes = eth2_network_config.unwrap().genesis_state_bytes.unwrap();
        let genesis_state =
            BeaconState::<MainnetEthSpec>::from_ssz_bytes(&genesis_state_bytes, &spec)
                .map_err(|e| format!("Unable to parse genesis state SSZ: {:?}", e))?;
        let enr_fork_id = spec.enr_fork_id::<MainnetEthSpec>(
            spec.genesis_slot,
            genesis_state.genesis_validators_root(),
        );
        let fork_context = Arc::new(ForkContext::new::<MainnetEthSpec>(
            spec.genesis_slot,
            genesis_state.genesis_validators_root(),
            &spec,
        ));

        let my_peer_id =
            PeerIdSerialized::from_str("16Uiu2HAkwgkdraX5wvaCkuRi1YdU5VUvpdQH42Un2DXyADYXAD8Q")
                .unwrap();

        network_config.trusted_peers.push(my_peer_id.clone());

        executor.spawn(
            async move {
                let service_context = Context {
                    config: &network_config,
                    enr_fork_id,
                    fork_context: fork_context.clone(),
                    chain_spec: &spec,
                    gossipsub_registry: None,
                };

                let (network_globals, mut libp2p) = Service::<MainnetEthSpec>::new(
                    context.executor.clone(),
                    service_context,
                    &network_log,
                )
                .await
                .unwrap();

                libp2p.send_request(
                    my_peer_id.into(),
                    RequestId::Sync(1),
                    Request::BlocksByRange(BlocksByRangeRequest {
                        start_slot: 100,
                        count: 10,
                        step: 1,
                    }),
                );
            },
            "network",
        );
        environment.block_until_shutdown_requested()?;

        Ok(())
    }
    */

    pub fn start_old() -> Result<(), String> {
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
        let genesis_state_bytes = eth2_network_config
            .clone()
            .unwrap()
            .genesis_state_bytes
            .unwrap();
        let mut environment = environment_builder
            .optional_eth2_network_config(eth2_network_config)?
            .initialize_logger(logger_config)?
            .multi_threaded_tokio_runtime()?
            .build()?;
        let context = environment.core_context();
        let executor = context.executor.clone();

        let client_config = get_config::<MainnetEthSpec>(&ArgMatches::default(), &context)?;
        let network_config = client_config.clone().network;
        let spec = context.eth2_config().spec.clone();

        let store =
            HotColdDB::<MainnetEthSpec, LevelDB<MainnetEthSpec>, LevelDB<MainnetEthSpec>>::open(
                &client_config.create_data_dir()?,
                &client_config.create_freezer_db_path()?,
                |_, _, _| Ok(()),
                client_config.clone().store,
                spec.clone(),
                context.log().clone(),
            )
            .map_err(|e| format!("Unable to open database: {:?}", e))?;

        let genesis_state =
            BeaconState::<MainnetEthSpec>::from_ssz_bytes(&genesis_state_bytes, &spec)
                .map_err(|e| format!("Unable to parse genesis state SSZ: {:?}", e))?;

        let clock = SystemTimeSlotClock::new(
            spec.genesis_slot,
            Duration::from_secs(genesis_state.genesis_time()),
            Duration::from_secs(spec.seconds_per_slot),
        );

        let beacon_chain_builder = BeaconChainBuilder::new(MainnetEthSpec::default());

        let beacon_chain = beacon_chain_builder
            .logger(context.log().clone())
            .store(store)
            .genesis_state(genesis_state)?
            .slot_clock(clock)
            .shutdown_sender(context.executor.shutdown_sender())
            .monitor_validators(false, vec![], context.log().clone())
            .eth1_backend(None as Option<DummyEth1ChainBackend<MainnetEthSpec>>)
            .build()?;

        let beacon_chain = Arc::new(beacon_chain);

        executor.spawn(
            async move {
                match NetworkService::start(
                    beacon_chain.clone(),
                    &network_config,
                    context.executor.clone(),
                    None,
                )
                .await
                {
                    Ok((network_globals, network_send)) => {
                        let client_config = client_config.clone();
                    }
                    Err(_) => {
                        let _ = context
                            .executor
                            .clone()
                            .shutdown_sender()
                            .try_send(ShutdownReason::Failure("Failed to start network}"));
                    }
                }
            },
            "beacon_indexer",
        );

        environment.block_until_shutdown_requested()?;

        Ok(())
    }
}
