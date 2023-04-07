use std::sync::Arc;

use lighthouse_types::EthSpec;
use parking_lot::RwLock;
use task_executor::TaskExecutor;
use tokio::sync::watch::Receiver;

use crate::beacon_chain::beacon_context::BeaconContext;

use self::{persist_block_worker::PersistBlockWorker, persist_epoch_worker::PersistEpochWorker};

pub mod persist_block_worker;
pub mod persist_epoch_worker;

pub struct Workers<E: EthSpec> {
    pub epoch_persister: PersistEpochWorker<E>,
    pub existing_block_persister: PersistBlockWorker<E>,
}

impl<E: EthSpec> Workers<E> {
    pub fn new(
        executor: &TaskExecutor,
        base_dir: String,
        beacon_context: Arc<BeaconContext<E>>,
        shutdown_trigger: Receiver<()>,
    ) -> Self {
        let beacon_state = Arc::new(RwLock::new(beacon_context.genesis_state.clone()));
        Self {
            epoch_persister: PersistEpochWorker::new(
                executor,
                base_dir.clone(),
                beacon_state.clone(),
                beacon_context.spec.clone(),
                shutdown_trigger.clone(),
            ),
            existing_block_persister: PersistBlockWorker::new(
                executor,
                base_dir,
                beacon_state,
                beacon_context.spec.clone(),
                shutdown_trigger,
            ),
        }
    }
}
