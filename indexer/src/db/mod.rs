use std::sync::Arc;

use lighthouse_types::{EthSpec, Hash256, Slot};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use types::block_request::BlockRequestModelWithId;

use self::{
    block_range_request_state::BlockRangeRequestState, blocks_by_epoch::BlocksByEpoch,
    latest_epoch::LatestEpoch, latest_slot::LatestSlot, proposed_block_roots::ProposedBlockRoots,
};

mod block_by_root_requests;
mod block_range_request_state;
mod blocks_by_epoch;
mod latest_epoch;
mod latest_slot;
mod proposed_block_roots;

pub use block_by_root_requests::BlockByRootRequests;

#[derive(Debug, Default)]
pub struct Stores<E: EthSpec> {
    latest_slot: RwLock<LatestSlot>,
    latest_epoch: Arc<RwLock<LatestEpoch>>,
    block_by_epoch: RwLock<BlocksByEpoch<E>>,
    proposed_block_roots: RwLock<ProposedBlockRoots>,
    block_range_request_state: RwLock<BlockRangeRequestState>,
    block_by_root_requests: RwLock<BlockByRootRequests>,
}

impl<E: EthSpec> Stores<E> {
    pub fn new(block_requests: Vec<BlockRequestModelWithId>) -> Self {
        let latest_epoch = Arc::new(RwLock::new(LatestEpoch::default()));

        Self {
            latest_epoch: latest_epoch.clone(),
            block_by_epoch: RwLock::new(BlocksByEpoch::new(latest_epoch)),
            block_by_root_requests: RwLock::new(BlockByRootRequests::from_block_requests(
                block_requests,
            )),
            ..Default::default()
        }
    }

    pub fn latest_slot(&self) -> RwLockReadGuard<LatestSlot> {
        self.latest_slot.read()
    }

    pub fn latest_epoch(&self) -> RwLockReadGuard<LatestEpoch> {
        self.latest_epoch.read()
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

    pub fn block_range_request_state(&self) -> RwLockReadGuard<BlockRangeRequestState> {
        self.block_range_request_state.read()
    }

    pub fn block_range_request_state_mut(&self) -> RwLockWriteGuard<BlockRangeRequestState> {
        self.block_range_request_state.write()
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
