use std::sync::Arc;

use lighthouse_types::EthSpec;

use crate::beacon_chain::beacon_context::BeaconContext;

use self::{
    persist_epoch_worker::PersistEpochWorker,
    persist_existing_block_worker::PersistExistingBlockWorker,
};

pub mod persist_epoch_worker;
pub mod persist_existing_block_worker;

pub struct Workers<E: EthSpec> {
    pub epoch_persister: PersistEpochWorker<E>,
    pub existing_block_persister: PersistExistingBlockWorker,
}

impl<E: EthSpec> Workers<E> {
    pub fn new(base_dir: String, beacon_context: Arc<BeaconContext<E>>) -> Self {
        Self {
            epoch_persister: PersistEpochWorker::new(
                base_dir.clone(),
                beacon_context.genesis_state.clone(),
                beacon_context.spec.clone(),
            ),
            existing_block_persister: PersistExistingBlockWorker::new(base_dir),
        }
    }
}
