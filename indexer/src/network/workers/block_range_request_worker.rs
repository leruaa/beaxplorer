use std::{collections::HashSet, sync::Arc};

use itertools::Itertools;
use lighthouse_network::{NetworkEvent, Response};
use lighthouse_types::Hash256;
use parking_lot::{RwLock, RwLockReadGuard};
use slog::{debug, warn, Logger};
use store::EthSpec;
use tokio::sync::mpsc::UnboundedSender;

use crate::network::{
    augmented_network_service::{NetworkCommand, RequestId},
    block_by_root_requests::BlockByRootRequests,
    block_range_request::{BlockRangeRequest, BlockRangeRequestState},
    peer_db::PeerDb,
    persist_service::PersistMessage,
};

pub struct BlockRangeRequestWorker<E: EthSpec> {
    peer_db: Arc<PeerDb<E>>,
    network_command_send: UnboundedSender<NetworkCommand>,
    persist_send: UnboundedSender<PersistMessage<E>>,
    block_range_request_state: BlockRangeRequest,
    block_by_root_requests: Arc<RwLock<BlockByRootRequests>>,
    proposed_block_roots: HashSet<Hash256>,
    log: Logger,
}

impl<E: EthSpec> BlockRangeRequestWorker<E> {
    pub fn new(
        peer_db: Arc<PeerDb<E>>,
        network_command_send: UnboundedSender<NetworkCommand>,
        persist_send: UnboundedSender<PersistMessage<E>>,
        block_by_root_requests: Arc<RwLock<BlockByRootRequests>>,
        log: Logger,
    ) -> Self {
        BlockRangeRequestWorker {
            peer_db,
            network_command_send,
            persist_send,
            block_range_request_state: BlockRangeRequest::new(),
            block_by_root_requests,
            proposed_block_roots: HashSet::new(),
            log,
        }
    }

    pub fn handle_event(&mut self, event: &NetworkEvent<RequestId, E>) {
        match &event {
            NetworkEvent::PeerConnectedOutgoing(peer_id) => {
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

                    let connected_good_peers = self
                        .peer_db
                        .get_connected_good_peers()
                        .into_iter()
                        .map(|(id, _)| id)
                        .collect::<Vec<_>>();

                    if !unknown_blocks.is_empty() {
                        for (slot, root) in unknown_blocks {
                            self.block_by_root_requests
                                .write()
                                .request_block_by_root::<E>(
                                    &slot,
                                    &root,
                                    &self.network_command_send,
                                    &connected_good_peers,
                                )
                        }

                        let _ =
                            RwLockReadGuard::map(self.block_by_root_requests.read(), |unlocked| {
                                self.persist_send
                                    .send(PersistMessage::BlockRequests(unlocked.into()))
                                    .map_err(|err| err.to_string())
                                    .unwrap();
                                &()
                            });
                    }

                    for message in self.block_range_request_state.block_found(block.clone()) {
                        self.persist_send
                            .send(PersistMessage::Block(message))
                            .map_err(|err| err.to_string())
                            .unwrap();
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
