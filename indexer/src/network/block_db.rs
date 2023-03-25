use std::{
    collections::{
        hash_map::{Entry, OccupiedEntry},
        HashMap, HashSet,
    },
    sync::Arc,
};

use lighthouse_network::PeerId;
use lighthouse_types::{Hash256, Slot};
use parking_lot::RwLock;
use types::utils::{BlockByRootRequestState, RequestAttempts};

pub struct BlockDb {
    proposed_block_roots: RwLock<HashSet<Hash256>>,
    block_range_request_state: RwLock<BlockRangeRequestState>,
    block_by_root_requests: RwLock<HashMap<Hash256, RequestAttempts>>,
}

impl BlockDb {
    pub fn new() -> Arc<Self> {
        let block_db = Self {
            proposed_block_roots: RwLock::new(HashSet::new()),
            block_range_request_state: RwLock::new(BlockRangeRequestState::Idle),
            block_by_root_requests: RwLock::new(HashMap::new()),
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

    pub fn block_by_root_request_exists(&self, root: &Hash256) -> bool {
        self.block_by_root_requests.read().contains_key(root)
    }

    pub fn for_each_pending_block_by_root_requests<F>(&self, f: F)
    where
        F: FnMut((&Hash256, &mut RequestAttempts)),
    {
        self.block_by_root_requests
            .write()
            .iter_mut()
            .filter(|(_, req)| req.state != BlockByRootRequestState::Found)
            .for_each(f)
    }

    pub fn with_found_block_root<F>(&self, root: Hash256, found_by: PeerId, mut f: F)
    where
        F: FnMut(OccupiedEntry<Hash256, RequestAttempts>),
    {
        if let Entry::Occupied(attempt) = self
            .block_by_root_requests
            .write()
            .entry(root)
            .and_modify(|attempts| {
                if attempts.found_by.is_none() {
                    attempts.found_by = Some(found_by);
                }

                attempts.state = BlockByRootRequestState::Found;
            })
        {
            f(attempt)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BlockRangeRequestState {
    Idle,
    AwaitingPeer,
    Requesting(PeerId),
}
