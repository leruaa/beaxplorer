use std::{collections::HashSet, sync::Arc};

use itertools::Itertools;
use lighthouse_network::{NetworkEvent, Response};
use lighthouse_types::Hash256;
use parking_lot::RwLock;
use slog::{debug, warn, Logger};
use store::EthSpec;
use tokio::sync::mpsc::UnboundedSender;

use crate::direct_indexer::BlockMessage;

use super::{
    augmented_network_service::{NetworkCommand, RequestId},
    block_by_root_requests::BlockByRootRequests,
    block_range_request::{BlockRangeRequest, BlockRangeRequestState},
};

use super::peer_db::PeerDb;

pub struct Worker<E: EthSpec> {
    peer_db: PeerDb<E>,
    network_command_send: UnboundedSender<NetworkCommand>,
    block_send: UnboundedSender<BlockMessage<E>>,
    block_range_request_state: BlockRangeRequest,
    block_by_root_requests: Arc<RwLock<BlockByRootRequests>>,
    proposed_block_roots: HashSet<Hash256>,
    log: Logger,
}

impl<E: EthSpec> Worker<E> {
    pub fn new(
        peer_db: PeerDb<E>,
        network_command_send: UnboundedSender<NetworkCommand>,
        block_send: UnboundedSender<BlockMessage<E>>,
        block_by_root_requests: Arc<RwLock<BlockByRootRequests>>,
        log: Logger,
    ) -> Self {
        Worker {
            peer_db,
            network_command_send,
            block_send,
            block_range_request_state: BlockRangeRequest::new(),
            block_by_root_requests,
            proposed_block_roots: HashSet::new(),
            log,
        }
    }

    pub fn handle_event(&mut self, event: &NetworkEvent<RequestId, E>) {
        match &event {
            NetworkEvent::PeerConnectedOutgoing(peer_id) => {
                if self.peer_db.is_known_great_peer(peer_id) {
                    self.peer_db.add_great_peer(*peer_id);
                }

                self.block_range_request_state
                    .peer_connected(peer_id, &self.network_command_send);
            }

            NetworkEvent::PeerDisconnected(peer_id) => {
                if self.block_range_request_state.matches(peer_id) {
                    debug!(self.log, "Range request cancelled");
                    self.request_block_range();
                }
            }

            NetworkEvent::RPCFailed {
                id: RequestId::Range(_),
                ..
            } => {
                self.request_block_range();
            }

            NetworkEvent::ResponseReceived {
                id: RequestId::Range(start_slot),
                response: Response::BlocksByRange(block),
                ..
            } => {
                if let Some(block) = block {
                    self.proposed_block_roots.insert(block.canonical_root());

                    let unknown_blocks = block
                        .message()
                        .body()
                        .attestations()
                        .iter()
                        .map(|a| (a.data.slot, a.data.beacon_block_root))
                        .dedup()
                        .filter(|(_, r)| !self.proposed_block_roots.contains(r))
                        .collect::<Vec<_>>();

                    for (slot, root) in unknown_blocks {
                        self.block_by_root_requests.write().request_block_by_root(
                            &slot,
                            &root,
                            &self.network_command_send,
                            &self.peer_db,
                        )
                    }

                    for message in self.block_range_request_state.block_found(block.clone()) {
                        self.block_send.send(message).unwrap();
                    }
                } else {
                    // A block range response has finished, request another one
                    debug!(self.log, "Range request completed"; "start slot" => start_slot);
                    self.request_block_range();
                }
            }

            _ => {}
        }
    }

    fn request_block_range(&mut self) {
        if let BlockRangeRequestState::Idle = self
            .block_range_request_state
            .request_block_range(&self.network_command_send, &self.peer_db)
        {
            warn!(self.log, "Unable to find a peer for a block range request");
        }
    }
}
