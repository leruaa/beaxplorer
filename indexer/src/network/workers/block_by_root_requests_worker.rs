use std::sync::Arc;

use lighthouse_network::{NetworkEvent, Response};
use lighthouse_types::EthSpec;
use parking_lot::{RwLock, RwLockReadGuard};
use slog::{info, Logger};
use tokio::sync::mpsc::UnboundedSender;

use crate::network::{
    augmented_network_service::{NetworkCommand, RequestId},
    block_by_root_requests::BlockByRootRequests,
    peer_db::PeerDb,
    persist_service::PersistMessage,
};

pub struct BlockByRootRequestsWorker<E: EthSpec> {
    peer_db: Arc<PeerDb<E>>,
    network_command_send: UnboundedSender<NetworkCommand>,
    persist_send: UnboundedSender<PersistMessage<E>>,
    block_by_root_requests: Arc<RwLock<BlockByRootRequests>>,
    log: Logger,
}

impl<E: EthSpec> BlockByRootRequestsWorker<E> {
    pub fn new(
        peer_db: Arc<PeerDb<E>>,
        network_command_send: UnboundedSender<NetworkCommand>,
        persist_send: UnboundedSender<PersistMessage<E>>,
        block_by_root_requests: Arc<RwLock<BlockByRootRequests>>,
        log: Logger,
    ) -> Self {
        Self {
            peer_db,
            network_command_send,
            persist_send,
            block_by_root_requests,
            log,
        }
    }

    pub fn handle_event(&mut self, event: &NetworkEvent<RequestId, E>) {
        match event {
            NetworkEvent::PeerConnectedOutgoing(peer_id) => {
                self.block_by_root_requests
                    .write()
                    .peer_connected(peer_id, &self.network_command_send);

                if self.peer_db.is_good_peer(peer_id) {
                    info!(self.log, "Good peer connected"; "peer" => ?peer_id);
                }
            }

            NetworkEvent::PeerDisconnected(peer_id) => {
                self.block_by_root_requests
                    .write()
                    .peer_disconnected(peer_id);

                if self.block_by_root_requests.read().count() > 0
                    && !self.peer_db.has_connected_good_peers()
                {
                    for peer_id in self.peer_db.get_good_peers().iter() {
                        self.network_command_send
                            .send(NetworkCommand::DialPeer(*peer_id))
                            .unwrap();
                    }
                }
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
                            info!(self.log, "An orphaned block has been found"; "peer" => ?peer_id, "slot" => block.message().slot(), "root" => %block.canonical_root());

                            self.persist_send
                                .send(PersistMessage::new_ophan_block(block.clone()))
                                .map_err(|err| err.to_string())
                                .unwrap();

                            self.peer_db.add_good_peer(*peer_id);

                            self.persist_send
                                .send(PersistMessage::GoodPeers(self.peer_db.as_ref().into()))
                                .map_err(|err| err.to_string())
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

    pub fn persist(&self) {
        let _ = RwLockReadGuard::map(self.block_by_root_requests.read(), |unlocked| {
            self.persist_send
                .send(PersistMessage::BlockRequests(unlocked.into()))
                .map_err(|err| err.to_string())
                .unwrap();
            &()
        });
    }
}
