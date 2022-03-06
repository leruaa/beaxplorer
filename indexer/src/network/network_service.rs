use std::{collections::HashMap, pin::Pin, sync::Arc, time::Duration};

use environment::RuntimeContext;
use eth2_network_config::Eth2NetworkConfig;
use futures::{Future, Stream, StreamExt};
use libp2p::{
    bandwidth::BandwidthLogging,
    core::{muxing::StreamMuxerBox, transport::Boxed},
    dns::TokioDnsConfig,
    swarm::{SwarmBuilder, SwarmEvent},
    tcp::TokioTcpConfig,
    Multiaddr, Transport,
};
use lighthouse_network::{
    peer_manager::Keypair,
    rpc::{RequestId, RPC},
    PeerId, Request,
};
use store::{BeaconState, ForkContext, MainnetEthSpec};
use tokio::{
    select,
    sync::mpsc::{self, UnboundedSender},
};

use super::request_handler::SafeRequestHandler;

type BoxedTransport = Boxed<(PeerId, StreamMuxerBox)>;

struct Executor(task_executor::TaskExecutor);

impl libp2p::core::Executor for Executor {
    fn exec(&self, f: Pin<Box<dyn Future<Output = ()> + Send>>) {
        self.0.spawn(f, "libp2p");
    }
}

pub struct NetworkService {
    connection_send: UnboundedSender<Multiaddr>,
    connected_peers: HashMap<PeerId, UnboundedSender<Request>>,
    request_handler: SafeRequestHandler,
}

impl NetworkService {
    pub fn new(
        context: RuntimeContext<MainnetEthSpec>,
        network_config: Eth2NetworkConfig,
    ) -> Result<Self, String> {
        let spec = context.eth2_config().spec.clone();
        let genesis_state_bytes = network_config.genesis_state_bytes.unwrap();
        let genesis_state =
            BeaconState::<MainnetEthSpec>::from_ssz_bytes(&genesis_state_bytes, &spec)
                .map_err(|e| format!("Unable to parse genesis state SSZ: {:?}", e))?;
        let fork_context = Arc::new(ForkContext::new::<MainnetEthSpec>(
            spec.genesis_slot,
            genesis_state.genesis_validators_root(),
            &spec,
        ));
        let executor = context.clone().executor;

        let (connection_send, mut connection_recv) = mpsc::unbounded_channel::<Multiaddr>();

        //let connected_peers = Arc::new(vec![]);
        let request_handler = SafeRequestHandler::new();
        let mut request_handler_mut = request_handler.clone();
        context.executor.spawn(
            async move {
                let local_key = Keypair::generate_ed25519();
                let local_peer_id = PeerId::from(local_key.public());
                let transport = Self::build_transport(local_key).unwrap();
                let behaviour = RPC::<MainnetEthSpec>::new(fork_context, executor.log().clone());
                let mut swarm = SwarmBuilder::new(transport, behaviour, local_peer_id)
                    .executor(Box::new(Executor(executor)))
                    .build();

                loop {
                    select! {
                        connection_event = connection_recv.recv() => {
                            match connection_event {
                                Some(message) => {
                                    println!("Dial to {:}", message);
                                    swarm.dial(message).unwrap();
                                }
                                _ => println!("All sender have dropped"),
                            }
                        },
                        swarm_event = swarm.select_next_some() => {
                            match swarm_event {
                                SwarmEvent::NewListenAddr{address, ..} => {
                                    println!("Listening on {:?}",address)
                                }
                                SwarmEvent::Behaviour(e)=> println!("Behaviour: {:?}", e.event),
                                SwarmEvent::ConnectionEstablished{peer_id, ..} => {
                                    request_handler_mut.activate(peer_id).await.unwrap();
                                    println!("Connected to {:?}",peer_id);
                                }
                                SwarmEvent::OutgoingConnectionError{error, ..} => {
                                    println!("Error {:?}", error)
                                }
                                SwarmEvent::ConnectionClosed { peer_id, .. } => println!("Connection to {:} closed", peer_id),
                                SwarmEvent::IncomingConnection { .. } => println!("Incoming connection"),
                                SwarmEvent::IncomingConnectionError { error, .. } => println!("Incoming connection error: {:?}", error),
                                SwarmEvent::BannedPeer { peer_id, endpoint } => todo!(),
                                SwarmEvent::ExpiredListenAddr { listener_id, address } => todo!(),
                                SwarmEvent::ListenerClosed { listener_id, addresses, reason } => todo!(),
                                SwarmEvent::ListenerError { error, .. } => println!("Listener error {:?}", error),
                                SwarmEvent::Dialing(_) => println!("Dialing"), }
                        },
                        request = request_handler_mut.next() => {
                            if let Some((peer_id, request)) = request {
                                swarm.behaviour_mut().send_request(
                                    peer_id,
                                    RequestId::Behaviour,
                                    request.into(),
                                );
                            }
                        }
                    }
                }
            },
            "swarm",
        );

        let indexer = Self {
            connection_send,
            connected_peers: HashMap::new(),
            request_handler,
        };

        Ok(indexer)
    }

    pub async fn send_request(&mut self, request: Request, peer_id: PeerId, multiaddr: Multiaddr) {
        let mut request_handler = self.request_handler.guard().await;
        let is_connected = self.connected_peers.contains_key(&peer_id);

        let tx = self
            .connected_peers
            .entry(peer_id)
            .or_insert_with(|| request_handler.create_channel(peer_id).unwrap());

        if !is_connected {
            self.connection_send
                .send(multiaddr)
                .map_err(|_| "Can't send message".to_string())
                .unwrap();
        }

        tx.send(request).unwrap();
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
}

impl Stream for NetworkService {
    type Item = ();

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        std::task::Poll::Pending
    }
}
