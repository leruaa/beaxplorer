use std::{fmt::Display, sync::Arc};

use libp2p::Multiaddr;
use lighthouse_network::{
    libp2p::swarm::dial_opts::DialOpts, rpc::StatusMessage, BehaviourEvent, Context, Libp2pEvent,
    NetworkConfig, NetworkGlobals, PeerId, Request, Response, Service,
};
use slog::{error, info, Logger, Value};
use store::{EnrForkId, Epoch, EthSpec, ForkContext, Hash256, Slot};
use task_executor::TaskExecutor;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::beacon_chain::beacon_context::BeaconContext;

pub struct AugmentedNetworkService<E: EthSpec> {
    network_recv: UnboundedReceiver<NetworkMessage>,
    behavior_send: UnboundedSender<BehaviourEvent<RequestId, E>>,
    enr_fork_id: EnrForkId,
    service: Service<RequestId, E>,
    log: Logger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestId {
    Range(u64),
    Block(Hash256),
}

impl Value for RequestId {
    fn serialize(
        &self,
        _record: &slog::Record,
        _key: slog::Key,
        serializer: &mut dyn slog::Serializer,
    ) -> slog::Result {
        match self {
            RequestId::Range(start_slot) => serializer.emit_u64("start slot", *start_slot),
            RequestId::Block(root) => serializer.emit_str("block", &root.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum NetworkMessage {
    SendRequest {
        peer_id: PeerId,
        request_id: RequestId,
        request: Box<Request>,
    },
    Dial(Multiaddr),
}

impl<E: EthSpec> AugmentedNetworkService<E> {
    pub async fn start(
        executor: TaskExecutor,
        beacon_context: &BeaconContext<E>,
    ) -> Result<
        (
            UnboundedSender<NetworkMessage>,
            UnboundedReceiver<BehaviourEvent<RequestId, E>>,
            Arc<NetworkGlobals<E>>,
        ),
        String,
    > {
        let network_log = executor.log().clone();
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

        network_config.upnp_enabled = false;

        // construct the libp2p service context
        let service_context = Context {
            config: &network_config,
            enr_fork_id: beacon_context.current_fork_id(),
            fork_context: fork_context.clone(),
            chain_spec: &beacon_context.spec,
            gossipsub_registry: None,
        };

        let (network_send, network_recv) = mpsc::unbounded_channel::<NetworkMessage>();
        let (behavior_send, behavior_recv) =
            mpsc::unbounded_channel::<BehaviourEvent<RequestId, E>>();

        // launch libp2p service
        let (network_globals, libp2p) =
            Service::new(executor.clone(), service_context, &network_log)
                .await
                .map_err(|err| err.to_string())?;

        let network_service = Self {
            network_recv,
            behavior_send,
            enr_fork_id: beacon_context.current_fork_id(),
            service: libp2p,
            log: network_log,
        };

        network_service.spawn(executor);

        Ok((network_send, behavior_recv, network_globals))
    }

    pub fn spawn(mut self, executor: TaskExecutor) {
        executor.spawn(
            async move {
                loop {
                    tokio::select! {
                        ev = self.service.next_event() => self.handle_event(ev).await,
                        Some(msg) = self.network_recv.recv() => self.handle_network_message(msg)
                    }
                }
            },
            "network",
        );
    }

    async fn handle_event(&mut self, event: Libp2pEvent<RequestId, E>) {
        if let Libp2pEvent::Behaviour(event) = event {
            match event {
                BehaviourEvent::RequestReceived {
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
                    self.behavior_send.send(event).unwrap();
                }
            }
        }
    }

    fn handle_network_message(&mut self, message: NetworkMessage) {
        match message {
            NetworkMessage::SendRequest {
                peer_id,
                request_id,
                request,
            } => self.send_request(peer_id, request_id, *request),
            NetworkMessage::Dial(addr) => self.dial(addr),
        }
    }

    fn send_request(&mut self, peer_id: PeerId, request_id: RequestId, request: Request) {
        self.service.send_request(peer_id, request_id, request)
    }

    fn dial(&mut self, dial_ops: impl Into<DialOpts> + Clone + Display) {
        match self.service.swarm.dial(dial_ops.clone()) {
            Ok(_) => info!(self.log, "Dialed {}", dial_ops),
            Err(err) => error!(self.log, "Dial failed: {:?}", err),
        }
    }
}
