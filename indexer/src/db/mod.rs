use std::{collections::HashSet, sync::Arc};

use lighthouse_network::{NetworkGlobals, PeerId};
use lighthouse_types::EthSpec;
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
mod peer_db;
mod proposed_block_roots;

pub use block_by_root_requests::BlockByRootRequests;
pub use peer_db::PeerDb;

#[derive(Debug)]
pub struct Stores<E: EthSpec> {
    latest_slot: RwLock<LatestSlot>,
    latest_epoch: Arc<RwLock<LatestEpoch>>,
    block_by_epoch: RwLock<BlocksByEpoch<E>>,
    proposed_block_roots: RwLock<ProposedBlockRoots>,
    block_range_request_state: RwLock<BlockRangeRequestState>,
    block_by_root_requests: RwLock<BlockByRootRequests>,
    peer_db: RwLock<PeerDb<E>>,
}

impl<E: EthSpec> Stores<E> {
    pub fn new(
        network_globals: Arc<NetworkGlobals<E>>,
        block_requests: Vec<BlockRequestModelWithId>,
        good_peers: HashSet<PeerId>,
    ) -> Self {
        let latest_epoch = Arc::new(RwLock::new(LatestEpoch::default()));

        Self {
            latest_slot: RwLock::default(),
            latest_epoch: latest_epoch.clone(),
            block_by_epoch: RwLock::new(BlocksByEpoch::new(latest_epoch)),
            proposed_block_roots: RwLock::default(),
            block_range_request_state: RwLock::default(),
            block_by_root_requests: RwLock::new(BlockByRootRequests::from_block_requests(
                block_requests,
            )),
            peer_db: RwLock::new(PeerDb::new(network_globals, good_peers)),
        }
    }

    pub fn latest_slot(&self) -> RwLockReadGuard<LatestSlot> {
        self.latest_slot.read()
    }

    pub fn latest_slot_mut(&self) -> RwLockWriteGuard<LatestSlot> {
        self.latest_slot.write()
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

    pub fn proposed_block_roots_mut(&self) -> RwLockWriteGuard<ProposedBlockRoots> {
        self.proposed_block_roots.write()
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

    pub fn peer_db(&self) -> RwLockReadGuard<PeerDb<E>> {
        self.peer_db.read()
    }

    pub fn peer_db_mut(&self) -> RwLockWriteGuard<PeerDb<E>> {
        self.peer_db.write()
    }
}
