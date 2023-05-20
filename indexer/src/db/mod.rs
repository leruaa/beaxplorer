use lighthouse_types::{BeaconState, ChainSpec, EthSpec};
use parking_lot::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
use std::convert::TryFrom;
use std::sync::Arc;
use types::{
    block_request::BlockRequestModelWithId,
    block_root::BlockRootModel,
    committee::CommitteeModel,
    deposit::ExecutionLayerDepositModel,
    meta::{DepositMeta, Meta},
    utils::{MetaCache, ModelCache},
};

use crate::beacon_chain::beacon_context::BeaconContext;

use self::indexing_state::IndexingState;

mod block_by_root_requests;
mod indexing_state;
mod peer_db;

pub use block_by_root_requests::BlockByRootRequests;
pub use peer_db::PeerDb;

pub struct Stores<E: EthSpec> {
    indexing_state: RwLock<IndexingState<E>>,
    block_by_root_requests: RwLock<BlockByRootRequests>,
    block_roots_cache: Arc<RwLock<ModelCache<BlockRootModel>>>,
    committees_cache: Arc<RwLock<ModelCache<Vec<CommitteeModel>>>>,
    meta_cache: Arc<RwLock<MetaCache>>,
}

impl<E: EthSpec> Stores<E> {
    pub fn new(
        base_dir: String,
        beacon_context: Arc<BeaconContext<E>>,
        block_requests: Vec<BlockRequestModelWithId>,
    ) -> Self {
        Self {
            indexing_state: RwLock::new(IndexingState::new(beacon_context)),
            block_by_root_requests: RwLock::new(BlockByRootRequests::from_block_requests(
                block_requests,
            )),
            block_roots_cache: Arc::new(RwLock::new(ModelCache::new(base_dir.clone()))),
            committees_cache: Arc::new(RwLock::new(ModelCache::new(base_dir.clone()))),
            meta_cache: Arc::new(RwLock::new(MetaCache::new(base_dir))),
        }
    }

    pub fn indexing_state(&self) -> RwLockReadGuard<IndexingState<E>> {
        self.indexing_state.read()
    }

    pub fn indexing_state_mut(&self) -> RwLockWriteGuard<IndexingState<E>> {
        self.indexing_state.write()
    }

    pub fn block_by_root_requests(&self) -> RwLockReadGuard<BlockByRootRequests> {
        self.block_by_root_requests.read()
    }

    pub fn block_by_root_requests_mut(&self) -> RwLockWriteGuard<BlockByRootRequests> {
        self.block_by_root_requests.write()
    }

    pub fn block_roots_cache(&self) -> &RwLock<ModelCache<BlockRootModel>> {
        self.block_roots_cache.as_ref()
    }

    pub fn committees_cache(&self) -> &RwLock<ModelCache<Vec<CommitteeModel>>> {
        self.committees_cache.as_ref()
    }

    pub fn meta_cache(&self) -> RwLockReadGuard<MetaCache> {
        self.meta_cache.read()
    }

    pub fn meta_cache_mut(&self) -> RwLockWriteGuard<MetaCache> {
        self.meta_cache.write()
    }

    pub fn beacon_state(&self) -> MappedRwLockReadGuard<BeaconState<E>> {
        RwLockReadGuard::map(self.indexing_state(), |indexing_state| {
            &indexing_state.beacon_state
        })
    }

    pub fn spec(&self) -> MappedRwLockReadGuard<ChainSpec> {
        RwLockReadGuard::map(self.indexing_state(), |indexing_state| &indexing_state.spec)
    }

    pub fn get_latest_deposit_block(&self) -> Option<u64> {
        let mut meta_cache = self.meta_cache_mut();
        let mut meta = meta_cache.get_or::<ExecutionLayerDepositModel>(Meta::deposit_default());
        let deposit_meta = <&mut DepositMeta>::try_from(&mut meta).ok();

        deposit_meta.and_then(|m| m.latest_block)
    }
}
