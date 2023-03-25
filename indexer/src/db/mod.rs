use lighthouse_types::{Hash256, Slot};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use self::{block_by_root_requests::BlockByRootRequests, latest_slot::LatestSlot};

mod block_by_root_requests;
pub mod blocks_by_epoch;
mod latest_slot;

#[derive(Default)]
pub struct Stores {
    latest_slot: RwLock<LatestSlot>,
    block_by_root_requests: RwLock<BlockByRootRequests>,
}

impl Stores {
    pub fn latest_slot(&self) -> RwLockReadGuard<LatestSlot> {
        self.latest_slot.read()
    }

    pub fn block_by_root_requests(&self) -> RwLockReadGuard<BlockByRootRequests> {
        self.block_by_root_requests.read()
    }

    pub fn block_by_root_requests_mut(&self) -> RwLockWriteGuard<BlockByRootRequests> {
        self.block_by_root_requests.write()
    }

    pub fn update(&self, slot: Slot, root: Hash256) {
        self.latest_slot.write().replace(slot);
        //self.proposed_block_roots.write().insert(root);
    }
}
