use lighthouse_types::EthSpec;
use task_executor::TaskExecutor;
use tracing::{info, instrument};
use types::{
    epoch::{EpochExtendedModelWithId, EpochModelWithId, EpochsMeta},
    persistable::Persistable,
};

use crate::types::consolidated_epoch::ConsolidatedEpoch;

pub fn spawn_persist_epoch_worker<E: EthSpec>(
    base_dir: String,
    epoch: ConsolidatedEpoch<E>,
    executor: &TaskExecutor,
) {
    executor.spawn(
        async move { persist_epoch(&base_dir, epoch) },
        "persist epoch worker",
    );
}

#[instrument(name = "EpochPersist", fields(duration), skip_all)]
fn persist_epoch<E: EthSpec>(base_dir: &str, epoch: ConsolidatedEpoch<E>) {
    info!(%epoch, "Persisting epoch");

    EpochModelWithId::from(&epoch).persist(base_dir);
    EpochExtendedModelWithId::from(&epoch).persist(base_dir);
    EpochsMeta::new(epoch.number() + 1).persist(base_dir);
}
