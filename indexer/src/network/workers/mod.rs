use std::sync::Arc;

use lighthouse_types::EthSpec;
use task_executor::TaskExecutor;
use tokio::sync::watch::Receiver;

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
    pub fn new(
        executor: &TaskExecutor,
        base_dir: String,
        beacon_context: Arc<BeaconContext<E>>,
        shutdown_trigger: Receiver<()>,
    ) -> Self {
        Self {
            epoch_persister: PersistEpochWorker::new(
                executor,
                base_dir.clone(),
                beacon_context.genesis_state.clone(),
                beacon_context.spec.clone(),
                shutdown_trigger,
            ),
            existing_block_persister: PersistExistingBlockWorker::new(base_dir),
        }
    }
}
