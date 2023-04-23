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
use tracing::{debug, warn};
use tracing_slog::TracingSlogDrain;

use crate::{beacon_chain::beacon_context::BeaconContext, db::PeerDb};

pub struct ConsensusNetwork<E: EthSpec> {
    service: Network<RequestId, E>,
    peer_db: PeerDb<E>,
    command_recv: UnboundedReceiver<NetworkCommand>,
    event_send: UnboundedSender<NetworkEvent<RequestId, E>>,
    enr_fork_id: EnrForkId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestId {
    Range,
    Block(Hash256),
}

#[derive(Debug, Clone)]
pub enum NetworkCommand {
    SendRequest {
        peer_id: PeerId,
        request_id: RequestId,
        request: Box<Request>,
    },
    ReportPeer(PeerId, &'static str),
}

impl<E: EthSpec> ConsensusNetwork<E> {
    pub async fn new(
        beacon_context: Arc<BeaconContext<E>>,
        good_peers: HashMap<PeerId, Multiaddr>,
        executor: &TaskExecutor,
    ) -> Result<Self, String> {
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
        let (libp2p, network_globals) =
            Network::new(executor.clone(), service_context, &network_log)
                .await
                .map_err(|err| err.to_string())?;

        let consensus_network = Self {
            service: libp2p,
            peer_db: PeerDb::new(network_globals, good_peers),
            command_recv,
            event_send,
            enr_fork_id: beacon_context.current_fork_id(),
        };

        Ok(consensus_network)
    }

    pub fn peer_db(&self) -> &PeerDb<E> {
        &self.peer_db
    }

    pub fn peer_db_mut(&mut self) -> &mut PeerDb<E> {
        &mut self.peer_db
    }

    pub async fn next_event(&mut self) -> NetworkEvent<RequestId, E> {
        let event = self.service.next_event().await;

        if let NetworkEvent::RequestReceived {
            peer_id,
            id,
            request: Request::Status(_),
        } = event
        {
            self.service.send_response(
                peer_id,
                id,
                Response::Status(StatusMessage {
                    fork_digest: self.enr_fork_id.fork_digest,
                    finalized_root: Hash256::zero(),
                    finalized_epoch: Epoch::new(0),
                    head_root: Hash256::zero(),
                    head_slot: Slot::new(0),
                }),
            )
        }

        event
    }

    pub fn send_range_request(&mut self, to: Option<PeerId>, start_slot: u64) {
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

    pub fn send_block_by_root_request(&mut self, to: PeerId, root: Hash256) {
        let request = Request::BlocksByRoot(BlocksByRootRequest {
            block_roots: vec![root].into(),
        });

        self.service
            .send_request(to, RequestId::Block(root), request);
    }

    pub fn report_peer(&mut self, peer_id: PeerId, reason: &'static str) {
        warn!(peer = %peer_id, "{}", reason);
        self.service
            .report_peer(&peer_id, PeerAction::Fatal, ReportSource::RPC, reason);
    }
}
