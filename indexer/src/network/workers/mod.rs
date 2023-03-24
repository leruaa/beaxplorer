use std::sync::Arc;

use lighthouse_types::EthSpec;

use crate::beacon_chain::beacon_context::BeaconContext;

use self::persist_epoch_worker::PersistEpochWorker;

pub mod block_by_root_requests_worker;
pub mod block_range_request_worker;
pub mod persist_epoch_worker;

pub struct Workers<E: EthSpec> {
    pub persist_epoch: PersistEpochWorker<E>,
}

impl<E: EthSpec> Workers<E> {
    pub fn new(base_dir: String, beacon_context: Arc<BeaconContext<E>>) -> Self {
        Self {
            persist_epoch: PersistEpochWorker::new(
                base_dir,
                beacon_context.genesis_state.clone(),
                beacon_context.spec.clone(),
            ),
        }
    }
}
