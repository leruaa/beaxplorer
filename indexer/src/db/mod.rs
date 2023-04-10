use std::sync::Arc;

use lighthouse_network::{Multiaddr, NetworkGlobals, PeerId};
use lighthouse_types::{BeaconState, ChainSpec, EthSpec};
use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use types::block_request::BlockRequestModelWithId;

use crate::beacon_chain::beacon_context::BeaconContext;

use self::{
    block_range_request_state::BlockRangeRequest, block_roots_cache::BlockRootsCache,
    indexing_state::IndexingState, latest_slot::LatestSlot,
    proposed_block_roots::ProposedBlockRoots,
};

mod block_by_root_requests;
mod block_range_request_state;
mod block_roots_cache;
mod indexing_state;
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
    indexing_state: RwLock<IndexingState<E>>,
    block_range_request: RwLock<BlockRangeRequest>,
    block_by_root_requests: RwLock<BlockByRootRequests>,
    peer_db: RwLock<PeerDb<E>>,
    block_roots_cache: Arc<RwLock<BlockRootsCache>>,
}

impl<E: EthSpec> Stores<E> {
    pub fn new(
        base_dir: String,
        network_globals: Arc<NetworkGlobals<E>>,
        beacon_context: Arc<BeaconContext<E>>,
        block_requests: Vec<BlockRequestModelWithId>,
        good_peers: Vec<(PeerId, Multiaddr)>,
    ) -> Self {
        Self {
            latest_slot: RwLock::default(),
            proposed_block_roots: RwLock::default(),
            indexing_state: RwLock::new(IndexingState::new(base_dir.clone(), beacon_context)),
            block_range_request: RwLock::default(),
            block_by_root_requests: RwLock::new(BlockByRootRequests::from_block_requests(
                block_requests,
            )),
            peer_db: RwLock::new(PeerDb::new(
                network_globals,
                good_peers.into_iter().collect(),
            )),
            block_roots_cache: Arc::new(RwLock::new(BlockRootsCache::new(base_dir))),
        }
    }

    pub fn indexing_state(&self) -> RwLockReadGuard<IndexingState<E>> {
        self.indexing_state.read()
    }

    pub fn indexing_state_mut(&self) -> RwLockWriteGuard<IndexingState<E>> {
        self.indexing_state.write()
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

    pub fn block_roots_cache(&self) -> Arc<RwLock<BlockRootsCache>> {
        self.block_roots_cache.clone()
    }

    pub fn beacon_state(&self) -> MappedRwLockReadGuard<BeaconState<E>> {
        RwLockReadGuard::map(self.indexing_state(), |indexing_state| {
            &indexing_state.beacon_state
        })
    }

    pub fn spec(&self) -> MappedRwLockReadGuard<ChainSpec> {
        RwLockReadGuard::map(self.indexing_state(), |indexing_state| &indexing_state.spec)
    }
}
