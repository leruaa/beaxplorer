use std::sync::Arc;

use lighthouse_network::{NetworkEvent, Response};
use lighthouse_types::EthSpec;
use parking_lot::RwLock;
use slog::{info, Logger};
use tokio::sync::mpsc::UnboundedSender;

use crate::direct_indexer::BlockMessage;

use super::{
    augmented_network_service::{NetworkCommand, RequestId},
    block_by_root_requests::BlockByRootRequests,
};

pub struct BlockByRootRequestsWorker<E: EthSpec> {
    network_command_send: UnboundedSender<NetworkCommand>,
    block_send: UnboundedSender<BlockMessage<E>>,
    block_by_root_requests: Arc<RwLock<BlockByRootRequests>>,
    log: Logger,
}

impl<E: EthSpec> BlockByRootRequestsWorker<E> {
    pub fn new(
        network_command_send: UnboundedSender<NetworkCommand>,
        block_send: UnboundedSender<BlockMessage<E>>,
        block_by_root_requests: Arc<RwLock<BlockByRootRequests>>,
        log: Logger,
    ) -> Self {
        Self {
            network_command_send,
            block_send,
            block_by_root_requests,
            log,
        }
    }

    pub fn handle_event(&mut self, event: &NetworkEvent<RequestId, E>) {
        match event {
            NetworkEvent::PeerConnectedOutgoing(peer_id) => self
                .block_by_root_requests
                .write()
                .peer_connected(peer_id, &self.network_command_send),

            NetworkEvent::PeerDisconnected(peer_id) => {
                self.block_by_root_requests
                    .write()
                    .peer_disconnected(peer_id);
            }

            NetworkEvent::RPCFailed {
                id: RequestId::Block(root),
                peer_id,
            } => {
                self.block_by_root_requests
                    .write()
                    .failed_request(root, peer_id);
            }

            NetworkEvent::ResponseReceived {
                peer_id,
                id: RequestId::Block(root),
                response: Response::BlocksByRoot(block),
            } => {
                if self.block_by_root_requests.read().exists(root) {
                    if let Some(block) = block {
                        if self.block_by_root_requests.write().block_found(root) {
                            info!(self.log, "An orphaned block has been found"; "slot" => block.message().slot(), "root" => %block.canonical_root());
                            //self.peer_db.add_great_peer(peer_id);
                            self.block_send
                                .send(BlockMessage::Orphaned(block.clone()))
                                .unwrap();
                        }
                    } else {
                        self.block_by_root_requests.write().block_not_found(root);
                    }
                }
            }

            _ => {}
        }
    }
}
