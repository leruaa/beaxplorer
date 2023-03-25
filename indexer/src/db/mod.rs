use std::collections::HashSet;

use lighthouse_types::{EthSpec, Hash256, Slot};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use self::{
    block_by_root_requests::BlockByRootRequests, blocks_by_epoch::BlocksByEpoch,
    latest_slot::LatestSlot, proposed_block_roots::ProposedBlockRoots,
};

mod block_by_root_requests;
mod blocks_by_epoch;
mod latest_slot;
mod proposed_block_roots;

pub use blocks_by_epoch::EpochToPersist;

#[derive(Default)]
pub struct Stores<E: EthSpec> {
    latest_slot: RwLock<LatestSlot>,
    block_by_epoch: RwLock<BlocksByEpoch<E>>,
    proposed_block_roots: RwLock<ProposedBlockRoots>,
    block_by_root_requests: RwLock<BlockByRootRequests>,
}

impl<E: EthSpec> Stores<E> {
    pub fn latest_slot(&self) -> RwLockReadGuard<LatestSlot> {
        self.latest_slot.read()
    }

    pub fn block_by_epoch(&self) -> RwLockReadGuard<BlocksByEpoch<E>> {
        self.block_by_epoch.read()
    }

    pub fn block_by_epoch_mut(&self) -> RwLockWriteGuard<BlocksByEpoch<E>> {
        self.block_by_epoch.write()
    }

    pub fn proposed_block_roots(&self) -> RwLockReadGuard<ProposedBlockRoots> {
        self.proposed_block_roots.read()
    }

    pub fn block_by_root_requests(&self) -> RwLockReadGuard<BlockByRootRequests> {
        self.block_by_root_requests.read()
    }

    pub fn block_by_root_requests_mut(&self) -> RwLockWriteGuard<BlockByRootRequests> {
        self.block_by_root_requests.write()
    }

    pub fn update(&self, slot: Slot, root: Hash256) {
        self.latest_slot.write().replace(slot);
        self.proposed_block_roots.write().insert(root);
    }
}
