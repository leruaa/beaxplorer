use std::{collections::HashSet, sync::Arc};

use lighthouse_network::PeerId;
use lighthouse_types::{Hash256, Slot};
use parking_lot::RwLock;

pub struct BlockDb {
    proposed_block_roots: RwLock<HashSet<Hash256>>,
    block_range_request_state: RwLock<BlockRangeRequestState>,
}

impl BlockDb {
    pub fn new() -> Arc<Self> {
        let block_db = Self {
            proposed_block_roots: RwLock::new(HashSet::new()),
            block_range_request_state: RwLock::new(BlockRangeRequestState::Idle),
        };

        Arc::new(block_db)
    }

    pub fn contain_block_root(&self, root: &Hash256) -> bool {
        self.proposed_block_roots.read().contains(root)
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

    pub fn update(&self, slot: Slot, root: Hash256) {
        self.proposed_block_roots.write().insert(root);
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
