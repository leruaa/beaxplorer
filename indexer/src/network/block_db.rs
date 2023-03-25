use std::{collections::HashSet, sync::Arc};

use lighthouse_network::PeerId;
use lighthouse_types::{Hash256, Slot};
use parking_lot::RwLock;

pub struct BlockDb {
    block_range_request_state: RwLock<BlockRangeRequestState>,
}

impl BlockDb {
    pub fn new() -> Arc<Self> {
        let block_db = Self {
            block_range_request_state: RwLock::new(BlockRangeRequestState::Idle),
        };

        Arc::new(block_db)
    }

    pub fn is_requesting_block_range(&self) -> bool {
        matches!(
            *self.block_range_request_state.read(),
            BlockRangeRequestState::Requesting(_)
        )
    }

    pub fn block_range_matches(&self, peer_id: &PeerId) -> bool {
        match *self.block_range_request_state.read() {
            BlockRangeRequestState::Requesting(requesting_peer_id) => {
                requesting_peer_id == *peer_id
            }
            _ => false,
        }
    }

    pub fn block_range_requesting(&self, peer_id: PeerId) {
        *self.block_range_request_state.write() = BlockRangeRequestState::Requesting(peer_id)
    }

    pub fn block_range_awaiting_peer(&self) {
        *self.block_range_request_state.write() = BlockRangeRequestState::AwaitingPeer
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BlockRangeRequestState {
    Idle,
    AwaitingPeer,
    Requesting(PeerId),
}
