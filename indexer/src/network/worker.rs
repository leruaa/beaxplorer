use std::collections::HashSet;

use itertools::Itertools;
use lighthouse_network::{BehaviourEvent, Response};
use lighthouse_types::Hash256;
use slog::{debug, info, warn, Logger};
use store::EthSpec;
use tokio::sync::mpsc::UnboundedSender;

use crate::direct_indexer::BlockMessage;

use super::{
    augmented_network_service::{NetworkMessage, RequestId},
    block_by_root_requests::BlockByRootRequests,
    block_range_request::{BlockRangeRequest, BlockRangeRequestState},
};

use super::peer_db::PeerDb;

pub struct Worker<E: EthSpec> {
    peer_db: PeerDb<E>,
    network_send: UnboundedSender<NetworkMessage>,
    block_send: UnboundedSender<BlockMessage<E>>,
    block_range_request_state: BlockRangeRequest,
    block_by_root_requests_state: BlockByRootRequests,
    proposed_block_roots: HashSet<Hash256>,
    log: Logger,
}

impl<E: EthSpec> Worker<E> {
    pub fn new(
        peer_db: PeerDb<E>,
        network_send: UnboundedSender<NetworkMessage>,
        block_send: UnboundedSender<BlockMessage<E>>,
        log: Logger,
    ) -> Self {
        Worker {
            peer_db,
            network_send,
            block_send,
            block_range_request_state: BlockRangeRequest::new(),
            block_by_root_requests_state: BlockByRootRequests::new(),
            proposed_block_roots: HashSet::new(),
            log,
        }
    }

    pub fn handle_event(&mut self, event: BehaviourEvent<RequestId, E>) {
        match event {
            BehaviourEvent::PeerConnectedOutgoing(peer_id) => {
                if self.peer_db.is_known_great_peer(&peer_id) {
                    self.peer_db.add_great_peer(peer_id);
                }

                self.block_range_request_state
                    .peer_connected(&peer_id, &self.network_send);
                self.block_by_root_requests_state
                    .peer_connected(&peer_id, &self.network_send)
            }

            BehaviourEvent::PeerDisconnected(peer_id) => {
                if self.block_range_request_state.matches(&peer_id) {
                    debug!(self.log, "Range request cancelled");
                    self.request_block_range();
                }
                self.block_by_root_requests_state
                    .peer_disconnected(&peer_id);
            }

            BehaviourEvent::RPCFailed {
                id: RequestId::Range(_),
                ..
            } => {
                self.request_block_range();
            }

            BehaviourEvent::RPCFailed {
                id: RequestId::Block(root),
                peer_id,
            } => {
                self.block_by_root_requests_state
                    .failed_request(&root, &peer_id);
            }

            BehaviourEvent::ResponseReceived {
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
                        self.block_by_root_requests_state.request_block_by_root(
                            &slot,
                            &root,
                            &self.network_send,
                            &self.peer_db,
                        )
                    }

                    for message in self.block_range_request_state.block_found(block) {
                        self.block_send.send(message).unwrap();
                    }
                } else {
                    // A block range response has finished, request another one
                    debug!(self.log, "Range request completed"; "start slot" => start_slot);
                    self.request_block_range();
                }
            }

            BehaviourEvent::ResponseReceived {
                peer_id,
                id: RequestId::Block(root),
                response: Response::BlocksByRoot(block),
            } => {
                if self.block_by_root_requests_state.exists(&root) {
                    if let Some(block) = block {
                        if self.block_by_root_requests_state.block_found(&root) {
                            info!(self.log, "An orphaned block has been found"; "slot" => block.message().slot(), "root" => %block.canonical_root());
                            if let Some(peer_info) = self.peer_db.get_peer_info(&peer_id) {
                                for a in peer_info.listening_addresses() {
                                    info!(self.log, "New great peer: {a:?}");
                                }
                            }

                            self.peer_db.add_great_peer(peer_id);
                            self.block_send.send(BlockMessage::Orphaned(block)).unwrap();
                        }
                    } else {
                        self.block_by_root_requests_state.block_not_found(&root);
                    }
                }
            }

            _ => {}
        }
    }

    pub fn notify(&self) {
        self.block_range_request_state.notify(&self.log);
        self.block_by_root_requests_state.notify(&self.log);
    }

    fn request_block_range(&mut self) {
        if let BlockRangeRequestState::Idle = self
            .block_range_request_state
            .request_block_range(&self.network_send, &self.peer_db)
        {
            warn!(self.log, "Unable to find a peer for a block range request");
        }
    }
}
