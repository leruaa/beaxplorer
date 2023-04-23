use std::{collections::HashMap, sync::Arc};

use lighthouse_network::{
    rpc::{BlocksByRangeRequest, BlocksByRootRequest, StatusMessage},
    service::Network,
    Context, Multiaddr, NetworkConfig, NetworkEvent, PeerAction, PeerId, ReportSource, Request,
    Response,
};
use slog::{o, Logger};
use store::{EnrForkId, Epoch, EthSpec, ForkContext, Hash256, Slot};
use task_executor::TaskExecutor;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tracing::{debug, info, warn};
use tracing_slog::TracingSlogDrain;

use crate::{beacon_chain::beacon_context::BeaconContext, db::PeerDb};

struct ConsensusNetwork<E: EthSpec> {
    service: Network<RequestId, E>,
    command_recv: UnboundedReceiver<NetworkCommand>,
    event_send: UnboundedSender<NetworkEvent<RequestId, E>>,
    peer_db: PeerDb<E>,
    enr_fork_id: EnrForkId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestId {
    Range,
    Block(Hash256),
}

#[derive(Debug, Clone)]
pub enum NetworkCommand {
    SendRangeRequest {
        peer_id: Option<PeerId>,
        start_slot: u64,
    },
    SendBlockByRootRequest {
        peer_id: Option<PeerId>,
        root: Hash256,
    },
    ReportPeer(PeerId, &'static str),
}

pub async fn spawn<E: EthSpec>(
    beacon_context: Arc<BeaconContext<E>>,
    good_peers: HashMap<PeerId, Multiaddr>,
    executor: &TaskExecutor,
) -> Result<
    (
        UnboundedSender<NetworkCommand>,
        UnboundedReceiver<NetworkEvent<RequestId, E>>,
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
    network_config.libp2p_nodes = good_peers.values().cloned().collect();
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
    let (libp2p, network_globals) = Network::new(executor.clone(), service_context, &network_log)
        .await
        .map_err(|err| err.to_string())?;

    let network_service = ConsensusNetwork {
        service: libp2p,
        command_recv,
        event_send,
        peer_db: PeerDb::new(network_globals, good_peers),
        enr_fork_id: beacon_context.current_fork_id(),
    };

    network_service.spawn(executor);

    Ok((command_send, event_recv))
}

impl<E: EthSpec> ConsensusNetwork<E> {
    pub fn spawn(mut self, executor: &TaskExecutor) {
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
        if let NetworkEvent::PeerConnectedOutgoing(peer_id) = event {
            if self.peer_db.is_good_peer(&peer_id) {
                info!(peer = %peer_id, "Good peer connected");
            }
        }

        if let NetworkEvent::RPCFailed { id, peer_id } = event {
            let reason = match id {
                RequestId::Range => "Range request failed",
                RequestId::Block(_) => "Block by root request",
            };
            if self.peer_db.is_good_peer(&peer_id) {
                warn!(peer = %peer_id, "Connection to good peer failed");
            }

            self.report_peer(peer_id, reason);
        }

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
            NetworkCommand::SendRangeRequest {
                peer_id,
                start_slot,
            } => self.send_range_request(peer_id, start_slot),
            NetworkCommand::SendBlockByRootRequest { peer_id, root } => {
                self.send_block_by_root_request(peer_id, root)
            }
            NetworkCommand::ReportPeer(peer_id, reason) => {
                warn!(peer = %peer_id, "{}", reason);
                self.service
                    .report_peer(&peer_id, PeerAction::Fatal, ReportSource::RPC, reason);
            }
        }
    }

    fn send_range_request(&mut self, to: Option<PeerId>, start_slot: u64) {
        match to.or_else(|| self.peer_db.get_best_connected_peer()) {
            Some(to) => {
                debug!(start_slot, "Send range request");

                let request = Request::BlocksByRange(BlocksByRangeRequest {
                    start_slot,
                    count: 32,
                });

                self.service.send_request(to, RequestId::Range, request);
            }
            None => {
                warn!("No peer available for a new range request");
            }
        }
    }

    fn send_block_by_root_request(&mut self, to: Option<PeerId>, root: Hash256) {
        let peers = match to {
            Some(to) => vec![to],
            None => self
                .peer_db
                .good_peers_iter()
                .connected()
                .cloned()
                .collect::<Vec<_>>(),
        };

        peers.into_iter().for_each(|peer_id| {
            let request = Request::BlocksByRoot(BlocksByRootRequest {
                block_roots: vec![root].into(),
            });

            self.service
                .send_request(peer_id, RequestId::Block(root), request);
        });
    }

    pub fn report_peer(&mut self, peer_id: PeerId, reason: &'static str) {
        warn!(peer = %peer_id, "{}", reason);
        self.service
            .report_peer(&peer_id, PeerAction::Fatal, ReportSource::RPC, reason);
    }
}
