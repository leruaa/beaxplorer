use std::sync::Arc;

use lighthouse_network::{Multiaddr, NetworkGlobals, PeerId};
use lighthouse_types::EthSpec;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use types::block_request::BlockRequestModelWithId;

use self::{
    block_range_request_state::BlockRangeRequest, latest_epoch::LatestEpoch,
    latest_slot::LatestSlot, proposed_block_roots::ProposedBlockRoots,
};

mod block_by_root_requests;
mod block_range_request_state;
mod latest_epoch;
mod latest_slot;
mod peer_db;
mod proposed_block_roots;

pub use block_by_root_requests::BlockByRootRequests;
pub use peer_db::PeerDb;

#[derive(Debug)]
pub struct Stores<E: EthSpec> {
    latest_slot: RwLock<LatestSlot>,
    proposed_block_roots: RwLock<ProposedBlockRoots>,
    block_range_request: RwLock<BlockRangeRequest>,
    block_by_root_requests: RwLock<BlockByRootRequests>,
    peer_db: RwLock<PeerDb<E>>,
}

impl<E: EthSpec> Stores<E> {
    pub fn new(
        network_globals: Arc<NetworkGlobals<E>>,
        block_requests: Vec<BlockRequestModelWithId>,
        good_peers: Vec<(PeerId, Multiaddr)>,
    ) -> Self {
        Self {
            latest_slot: RwLock::default(),
            proposed_block_roots: RwLock::default(),
            block_range_request: RwLock::default(),
            block_by_root_requests: RwLock::new(BlockByRootRequests::from_block_requests(
                block_requests,
            )),
            peer_db: RwLock::new(PeerDb::new(
                network_globals,
                good_peers.into_iter().collect(),
            )),
        }
    }

    pub fn latest_slot(&self) -> RwLockReadGuard<LatestSlot> {
        self.latest_slot.read()
    }

    pub fn latest_slot_mut(&self) -> RwLockWriteGuard<LatestSlot> {
        self.latest_slot.write()
    }

    pub fn proposed_block_roots(&self) -> RwLockReadGuard<ProposedBlockRoots> {
        self.proposed_block_roots.read()
    }

    pub fn proposed_block_roots_mut(&self) -> RwLockWriteGuard<ProposedBlockRoots> {
        self.proposed_block_roots.write()
    }

    pub fn block_range_request(&self) -> RwLockReadGuard<BlockRangeRequest> {
        self.block_range_request.read()
    }

    pub fn block_range_request_mut(&self) -> RwLockWriteGuard<BlockRangeRequest> {
        self.block_range_request.write()
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
