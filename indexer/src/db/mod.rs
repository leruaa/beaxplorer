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
    utils::{MetaCache, ModelCache}, DeserializeOwned, path::FromPath, validator::{ValidatorModel, ValidatorExtendedModel},
};

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
    validators_cache: Arc<RwLock<ModelCache<ValidatorModel>>>,
    validators_extended_cache: Arc<RwLock<ModelCache<ValidatorExtendedModel>>>,
    meta_cache: Arc<RwLock<MetaCache>>,
}

impl<E: EthSpec + DeserializeOwned> Stores<E> {
    pub fn new(
        base_dir: String,
        genesis_state: BeaconState<E>,
        deposit_contract_deploy_block: u64,
        block_requests: Vec<BlockRequestModelWithId>,
    ) -> Self {
        let indexing_state = IndexingState::from_path( &base_dir, &()).unwrap_or_else(|_| IndexingState::new(genesis_state, deposit_contract_deploy_block));
        Self {
            indexing_state: RwLock::new(indexing_state),
            block_by_root_requests: RwLock::new(BlockByRootRequests::from_block_requests(
                block_requests,
            )),
            block_roots_cache: Arc::new(RwLock::new(ModelCache::new(base_dir.clone()))),
            committees_cache: Arc::new(RwLock::new(ModelCache::new(base_dir.clone()))),
            validators_cache: Arc::new(RwLock::new(ModelCache::new(base_dir.clone()))),
            validators_extended_cache: Arc::new(RwLock::new(ModelCache::new(base_dir.clone()))),
            meta_cache: Arc::new(RwLock::new(MetaCache::new(base_dir))),
        }
    }
}

impl<E: EthSpec> Stores<E> {
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

    pub fn validators_cache(&self) -> &RwLock<ModelCache<ValidatorModel>> {
        self.validators_cache.as_ref()
    }

    pub fn validators_extended_cache(&self) -> &RwLock<ModelCache<ValidatorExtendedModel>> {
        self.validators_extended_cache.as_ref()
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

    pub fn beacon_state_mut(&self) -> MappedRwLockWriteGuard<BeaconState<E>> {
        RwLockWriteGuard::map(self.indexing_state_mut(), |indexing_state| {
            &mut indexing_state.beacon_state
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
