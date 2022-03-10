use std::{
    collections::HashMap,
    marker::PhantomData,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};

use environment::RuntimeContext;
use eth2_network_config::Eth2NetworkConfig;
use futures::{future, join, Future, FutureExt, Stream, StreamExt};
use libp2p::{
    bandwidth::BandwidthLogging,
    core::{muxing::StreamMuxerBox, transport::Boxed},
    dns::TokioDnsConfig,
    swarm::{SwarmBuilder, SwarmEvent},
    tcp::TokioTcpConfig,
    Multiaddr, Swarm, Transport,
};
use lighthouse_network::{
    peer_manager::Keypair,
    rpc::{methods::Ping, outbound::OutboundRequest, RPCReceived, RequestId, RPC},
    PeerId, Request,
};
use slog::{error, info, Logger};
use store::{BeaconState, ForkContext, MainnetEthSpec};
use tokio::sync::{mpsc::UnboundedSender, Mutex};

use super::request_handler::SafeRequestHandler;

type BoxedTransport = Boxed<(PeerId, StreamMuxerBox)>;

struct Executor(task_executor::TaskExecutor);

impl libp2p::core::Executor for Executor {
    fn exec(&self, f: Pin<Box<dyn Future<Output = ()> + Send>>) {
        self.0.spawn(f, "libp2p");
    }
}

pub enum NetworkEvent {
    None,
}

pub struct NetworkService {
    swarm: Arc<Mutex<Swarm<RPC<MainnetEthSpec>>>>,
    connected_peers: HashMap<PeerId, UnboundedSender<OutboundRequest<MainnetEthSpec>>>,
    request_handler: SafeRequestHandler,
    log: Logger,
}

impl NetworkService {
    pub async fn new(
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

        //let connected_peers = Arc::new(vec![]);
        let request_handler = SafeRequestHandler::new();

        let local_key = Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        let transport = Self::build_transport(local_key).unwrap();
        let behaviour = RPC::<MainnetEthSpec>::new(fork_context, executor.log().clone());
        let swarm = SwarmBuilder::new(transport, behaviour, local_peer_id)
            .executor(Box::new(Executor(executor)))
            .build();

        let indexer = Self {
            swarm: Arc::new(Mutex::new(swarm)),
            connected_peers: HashMap::new(),
            request_handler,
            log: context.log().clone(),
        };

        Ok(indexer)
    }

    pub async fn connect(
        &mut self,
        peer_id: PeerId,
        multiaddr: &Multiaddr,
    ) -> &mut UnboundedSender<OutboundRequest<MainnetEthSpec>> {
        let mut request_handler = self.request_handler.guard().await;
        let is_connected = self.connected_peers.contains_key(&peer_id);

        let tx = self
            .connected_peers
            .entry(peer_id)
            .or_insert_with(|| request_handler.create_channel(peer_id).unwrap());

        if !is_connected {
            self.swarm.lock().await.dial(multiaddr.clone()).unwrap();
        }

        tx
    }

    pub async fn send_request(
        &self,
        request: OutboundRequest<MainnetEthSpec>,
        peer_id: PeerId,
    ) -> Result<(), String> {
        let tx = self.connected_peers.get(&peer_id).ok_or("err")?;
        tx.send(request).map_err(|err| err.to_string())?;

        Ok(())
    }

    async fn poll_requests(&self) {
        let mut request_handler = self.request_handler.guard().await;

        while let Some((peer_id, r)) = request_handler.next().await {
            info!(self.log, "Sending request to {:?}", peer_id);
            self.swarm.lock().await.behaviour_mut().send_request(
                peer_id,
                RequestId::Behaviour,
                r.into(),
            );
        }
    }

    async fn poll_swarm(&self) -> Option<NetworkEvent> {
        match self.swarm.lock().await.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on {:?}", address);
                Some(NetworkEvent::None)
            }
            SwarmEvent::Behaviour(e) => {
                match e.event {
                    Ok(event) => match event {
                        RPCReceived::Request(_, request) => match request.protocol() {
                            lighthouse_network::rpc::Protocol::Status => todo!(),
                            lighthouse_network::rpc::Protocol::Goodbye => todo!(),
                            lighthouse_network::rpc::Protocol::BlocksByRange => todo!(),
                            lighthouse_network::rpc::Protocol::BlocksByRoot => todo!(),
                            lighthouse_network::rpc::Protocol::Ping => {
                                let request = OutboundRequest::Ping(Ping { data: 1 });
                                self.send_request(request, e.peer_id).await.unwrap();
                            }
                            lighthouse_network::rpc::Protocol::MetaData => {
                                let request = OutboundRequest::MetaData(PhantomData);
                                self.send_request(request, e.peer_id).await.unwrap();
                            }
                        },
                        RPCReceived::Response(_, response) => {
                            info!(self.log, "Response: {:?}", response)
                        }
                        RPCReceived::EndOfStream(_, _) => todo!(),
                    },
                    Err(err) => error!(self.log, "{:?}", err),
                }
                Some(NetworkEvent::None)
            }
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                self.request_handler
                    .guard()
                    .await
                    .activate(peer_id)
                    .unwrap();
                info!(self.log, "Connected to {:?}", peer_id);
                Some(NetworkEvent::None)
            }
            SwarmEvent::OutgoingConnectionError { error, .. } => {
                println!("Error {:?}", error);
                Some(NetworkEvent::None)
            }
            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                self.request_handler.guard().await.close_channel(peer_id);
                info!(self.log, "Connection to {:} closed", peer_id);
                Some(NetworkEvent::None)
            }
            SwarmEvent::IncomingConnection { .. } => {
                println!("Incoming connection");
                Some(NetworkEvent::None)
            }
            SwarmEvent::IncomingConnectionError { error, .. } => {
                println!("Incoming connection error: {:?}", error);
                Some(NetworkEvent::None)
            }
            SwarmEvent::BannedPeer { peer_id, endpoint } => todo!(),
            SwarmEvent::ExpiredListenAddr {
                listener_id,
                address,
            } => todo!(),
            SwarmEvent::ListenerClosed {
                listener_id,
                addresses,
                reason,
            } => todo!(),
            SwarmEvent::ListenerError { error, .. } => {
                println!("Listener error {:?}", error);
                Some(NetworkEvent::None)
            }
            SwarmEvent::Dialing(_) => {
                println!("Dialing");
                Some(NetworkEvent::None)
            }
        }
    }

    async fn poll_network(&self) -> Option<NetworkEvent> {
        let (r, _) = join!(self.poll_swarm(), self.poll_requests());
        r
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
    type Item = NetworkEvent;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let future = self.poll_network();
        if let Poll::Ready(p) = Box::pin(future).poll_unpin(cx) {
            match p {
                Some(p) => Poll::Ready(Some(p)),
                None => Poll::Pending,
            }
        } else {
            Poll::Pending
        }
    }
}
