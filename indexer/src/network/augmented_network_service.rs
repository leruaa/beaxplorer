use std::sync::Arc;

use lighthouse_network::{
    rpc::StatusMessage, service::Network, Context, Multiaddr, NetworkConfig, NetworkEvent,
    NetworkGlobals, PeerId, Request, Response,
};
use slog::{o, Logger};
use store::{EnrForkId, Epoch, EthSpec, ForkContext, Hash256, Slot};
use task_executor::TaskExecutor;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tracing::info;
use tracing_slog::TracingSlogDrain;

use crate::beacon_chain::beacon_context::BeaconContext;

pub struct AugmentedNetworkService<E: EthSpec> {
    command_recv: UnboundedReceiver<NetworkCommand>,
    event_send: UnboundedSender<NetworkEvent<RequestId, E>>,
    enr_fork_id: EnrForkId,
    service: Network<RequestId, E>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestId {
    Range(u64),
    Block(Hash256),
}

#[derive(Debug, Clone)]
pub enum NetworkCommand {
    SendRequest {
        peer_id: PeerId,
        request_id: RequestId,
        request: Box<Request>,
    },
    DialPeer(PeerId),
}

impl<E: EthSpec> AugmentedNetworkService<E> {
    pub async fn start(
        executor: TaskExecutor,
        beacon_context: Arc<BeaconContext<E>>,
        known_peers: Vec<Multiaddr>,
    ) -> Result<
        (
            UnboundedSender<NetworkCommand>,
            UnboundedReceiver<NetworkEvent<RequestId, E>>,
            Arc<NetworkGlobals<E>>,
        ),
        String,
    > {
        let network_log = Logger::root(TracingSlogDrain, o!());

        let mut network_config = NetworkConfig::default();

        let genesis_validators_root = beacon_context.genesis_state.genesis_validators_root();

        // Create a fork context for the given config and genesis validators root
        let fork_context = Arc::new(ForkContext::new::<E>(
            beacon_context.current_slot(),
            genesis_validators_root,
            &beacon_context.spec,
        ));

        if let Some(boot_nodes) = &beacon_context.eth2_network_config.boot_enr {
            network_config.boot_nodes_enr.extend_from_slice(boot_nodes)
        }
        network_config.libp2p_nodes = known_peers;
        /*
        network_config.libp2p_nodes = vec![
            "/ip4/51.79.202.73/tcp/13000".parse().unwrap(),
            "/ip4/76.141.229.155/tcp/13000".parse().unwrap(),
            "/ip4/178.128.188.228/tcp/13000".parse().unwrap(),
            "/ip4/107.184.229.134/tcp/13000".parse().unwrap(),
            "/ip4/76.69.229.226/tcp/13000".parse().unwrap(),
            "/ip4/67.174.112.67/tcp/13000".parse().unwrap(),
            "/ip4/8.9.30.14/tcp/13000".parse().unwrap(),
            "/ip4/173.174.120.56/tcp/13000".parse().unwrap(),
            "/ip4/34.230.190.149/tcp/13000".parse().unwrap(),
            "/ip4/98.0.57.197/tcp/13103".parse().unwrap(),
            "/ip4/98.13.141.186/tcp/13103".parse().unwrap(),
            "/ip4/204.13.164.143/tcp/13000".parse().unwrap(),
            "/ip4/104.186.143.194/tcp/13000".parse().unwrap(),
            "/ip4/54.65.63.75/tcp/13000".parse().unwrap(),
            "/ip4/15.164.101.121/tcp/13000".parse().unwrap(),
            "/ip4/121.78.247.249/tcp/13000".parse().unwrap(),
            "/ip4/209.151.145.125/tcp/13000".parse().unwrap(),
            "/ip4/95.111.198.189/tcp/13000".parse().unwrap(),
            "/ip4/66.42.64.100/tcp/13000".parse().unwrap(),
            "/ip4/139.9.74.98/tcp/13000".parse().unwrap(),
            "/ip4/178.128.13.206/tcp/13000".parse().unwrap(),
            "/ip4/99.130.254.231/tcp/13000".parse().unwrap(),
            "/ip4/76.93.16.249/tcp/13000".parse().unwrap(),
        ];
         */

        network_config.upnp_enabled = false;

        // construct the libp2p service context
        let service_context = Context {
            config: &network_config,
            enr_fork_id: beacon_context.current_fork_id(),
            fork_context: fork_context.clone(),
            chain_spec: &beacon_context.spec,
            gossipsub_registry: None,
        };

        let (command_send, command_recv) = mpsc::unbounded_channel::<NetworkCommand>();
        let (event_send, event_recv) = mpsc::unbounded_channel::<NetworkEvent<RequestId, E>>();

        // launch libp2p service
        let (libp2p, network_globals) =
            Network::new(executor.clone(), service_context, &network_log)
                .await
                .map_err(|err| err.to_string())?;

        let network_service = Self {
            command_recv,
            event_send,
            enr_fork_id: beacon_context.current_fork_id(),
            service: libp2p,
        };

        network_service.spawn(executor);

        Ok((command_send, event_recv, network_globals))
    }

    pub fn spawn(mut self, executor: TaskExecutor) {
        executor.spawn(
            async move {
                loop {
                    tokio::select! {
                        ev = self.service.next_event() => self.handle_event(ev).await,
                        Some(msg) = self.command_recv.recv() => self.handle_network_message(msg)
                    }
                }
            },
            "network",
        );
    }

    async fn handle_event(&mut self, event: NetworkEvent<RequestId, E>) {
        match event {
            NetworkEvent::RequestReceived {
                peer_id,
                id,
                request: Request::Status(_),
            } => self.service.send_response(
                peer_id,
                id,
                Response::Status(StatusMessage {
                    fork_digest: self.enr_fork_id.fork_digest,
                    finalized_root: Hash256::zero(),
                    finalized_epoch: Epoch::new(0),
                    head_root: Hash256::zero(),
                    head_slot: Slot::new(0),
                }),
            ),

            event => {
                self.event_send.send(event).unwrap();
            }
        }
    }

    fn handle_network_message(&mut self, message: NetworkCommand) {
        match message {
            NetworkCommand::SendRequest {
                peer_id,
                request_id,
                request,
            } => self.send_request(peer_id, request_id, *request),
            NetworkCommand::DialPeer(peer_id) => self.dial(&peer_id),
        }
    }

    fn send_request(&mut self, peer_id: PeerId, request_id: RequestId, request: Request) {
        self.service.send_request(peer_id, request_id, request)
    }

    fn dial(&mut self, peer_id: &PeerId) {
        if !self.service.peer_manager().is_connected(peer_id) {
            info!("Dialing {peer_id:?}");
            self.service.peer_manager_mut().dial_peer(peer_id, None)
        }
    }
}
