use std::sync::Arc;

use lighthouse_network::{Multiaddr, NetworkGlobals, PeerId};
use lighthouse_types::{BeaconState, ChainSpec, EthSpec};
use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use types::{
    block_request::BlockRequestModelWithId, block_root::BlockRootModel, committee::CommitteeModel,
    utils::ModelCache,
};

use crate::beacon_chain::beacon_context::BeaconContext;

use self::{block_range_requests::BlockRangeRequests, indexing_state::IndexingState};

mod block_by_root_requests;
mod block_range_requests;
mod indexing_state;
mod peer_db;

pub use block_by_root_requests::BlockByRootRequests;
pub use peer_db::PeerDb;

pub struct Stores<E: EthSpec> {
    indexing_state: RwLock<IndexingState<E>>,
    block_range_requests: RwLock<BlockRangeRequests<E>>,
    block_by_root_requests: RwLock<BlockByRootRequests>,
    peer_db: RwLock<PeerDb<E>>,
    block_roots_cache: Arc<RwLock<ModelCache<BlockRootModel>>>,
    committees_cache: Arc<RwLock<ModelCache<Vec<CommitteeModel>>>>,
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
            indexing_state: RwLock::new(IndexingState::new(beacon_context)),
            block_range_requests: RwLock::default(),
            block_by_root_requests: RwLock::new(BlockByRootRequests::from_block_requests(
                block_requests,
            )),
            peer_db: RwLock::new(PeerDb::new(
                network_globals,
                good_peers.into_iter().collect(),
            )),
            block_roots_cache: Arc::new(RwLock::new(ModelCache::new(base_dir.clone()))),
            committees_cache: Arc::new(RwLock::new(ModelCache::new(base_dir))),
        }
    }

    pub fn indexing_state(&self) -> RwLockReadGuard<IndexingState<E>> {
        self.indexing_state.read()
    }

    pub fn indexing_state_mut(&self) -> RwLockWriteGuard<IndexingState<E>> {
        self.indexing_state.write()
    }

    pub fn block_range_requests(&self) -> RwLockReadGuard<BlockRangeRequests<E>> {
        self.block_range_requests.read()
    }

    pub fn block_range_requests_mut(&self) -> RwLockWriteGuard<BlockRangeRequests<E>> {
        self.block_range_requests.write()
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

    pub fn block_roots_cache(&self) -> Arc<RwLock<ModelCache<BlockRootModel>>> {
        self.block_roots_cache.clone()
    }

    pub fn committees_cache(&self) -> Arc<RwLock<ModelCache<Vec<CommitteeModel>>>> {
        self.committees_cache.clone()
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
