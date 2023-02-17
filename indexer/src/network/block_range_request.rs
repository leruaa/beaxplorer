use std::sync::Arc;

use lighthouse_network::{rpc::BlocksByRangeRequest, PeerId, Request};
use lighthouse_types::{EthSpec, SignedBeaconBlock, Slot};
use slog::{debug, Logger};
use tokio::sync::mpsc::UnboundedSender;

use crate::direct_indexer::BlockMessage;

use super::{
    augmented_network_service::{NetworkMessage, RequestId},
    peer_db::PeerDb,
};

#[derive(Debug, Clone, Copy)]
pub enum BlockRangeRequestState {
    Idle,
    AwaitingPeer,
    Requesting(PeerId),
}

pub struct BlockRangeRequest {
    state: BlockRangeRequestState,
    latest_slot: Option<Slot>,
}

impl BlockRangeRequest {
    pub fn new() -> Self {
        BlockRangeRequest {
            state: BlockRangeRequestState::Idle,
            latest_slot: None,
        }
    }

    pub fn matches(&self, peer_id: &PeerId) -> bool {
        match self.state {
            BlockRangeRequestState::Requesting(requesting_peer_id) => {
                requesting_peer_id == *peer_id
            }
            _ => false,
        }
    }

    pub fn peer_connected(
        &mut self,
        peer_id: &PeerId,
        network_send: &UnboundedSender<NetworkMessage>,
    ) {
        match self.state {
            BlockRangeRequestState::Idle | BlockRangeRequestState::AwaitingPeer => {
                let start_slot = match self.latest_slot {
                    Some(s) => s.as_u64() + 1,
                    None => 0,
                };
                network_send
                    .send(NetworkMessage::SendRequest {
                        peer_id: *peer_id,
                        request_id: RequestId::Range(start_slot),
                        request: Box::new(Request::BlocksByRange(BlocksByRangeRequest {
                            start_slot,
                            count: 32,
                        })),
                    })
                    .unwrap();
            }
            _ => {}
        }
    }

    pub fn block_found<E: EthSpec>(
        &mut self,
        block: Arc<SignedBeaconBlock<E>>,
    ) -> Vec<BlockMessage<E>> {
        let previous_latest_slot = self.latest_slot.unwrap_or_else(|| Slot::new(0));
        self.latest_slot = Some(block.message().slot());

        let mut messages = (previous_latest_slot.as_u64()
            ..self.latest_slot.unwrap_or_else(|| Slot::new(0)).as_u64())
            .map(Slot::new)
            .map(BlockMessage::Missed)
            .collect::<Vec<_>>();

        messages.push(BlockMessage::Proposed(block));

        messages
    }

    pub fn request_block_range<E: EthSpec>(
        &mut self,
        network_send: &UnboundedSender<NetworkMessage>,
        peer_db: &PeerDb<E>,
    ) -> BlockRangeRequestState {
        if let Some(peer_id) = peer_db.get_best_connected_peer() {
            let start_slot = match self.latest_slot {
                Some(s) => s.as_u64() + 1,
                None => 0,
            };
            self.state = BlockRangeRequestState::Requesting(peer_id);

            network_send
                .send(NetworkMessage::SendRequest {
                    peer_id,
                    request_id: RequestId::Range(start_slot),
                    request: Box::new(Request::BlocksByRange(BlocksByRangeRequest {
                        start_slot,
                        count: 32,
                    })),
                })
                .unwrap();
        } else {
            self.state = BlockRangeRequestState::AwaitingPeer;
        }

        self.state
    }

    pub fn notify(&self, log: &Logger) {
        debug!(log, "Block range status"; "latest slot" => self.latest_slot);
    }
}
