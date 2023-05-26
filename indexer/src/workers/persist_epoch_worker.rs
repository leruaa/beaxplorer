use std::sync::Arc;

use lighthouse_types::EthSpec;
use task_executor::TaskExecutor;
use tracing::{info, instrument};
use types::{
    epoch::{EpochExtendedModelWithId, EpochModel, EpochModelWithId},
    persistable::ResolvablePersistable,
};

use crate::{db::Stores, types::consolidated_epoch::ConsolidatedEpoch};

pub fn spawn_persist_epoch_worker<E: EthSpec>(
    base_dir: String,
    epoch: ConsolidatedEpoch<E>,
    stores: &Arc<Stores<E>>,
    executor: &TaskExecutor,
) {
    let stores = stores.clone();

    executor.spawn(
        async move { persist_epoch(&base_dir, epoch, &stores) },
        "persist epoch worker",
    );
}

#[instrument(name = "EpochPersist", fields(duration), skip_all)]
fn persist_epoch<E: EthSpec>(base_dir: &str, epoch: ConsolidatedEpoch<E>, stores: &Arc<Stores<E>>) {
    info!(%epoch, "Persisting epoch");

    EpochModelWithId::from(&epoch).save(base_dir).unwrap();
    EpochExtendedModelWithId::from(&epoch)
        .save(base_dir)
        .unwrap();

    stores
        .meta_cache_mut()
        .entry::<EpochModel>()
        .update_count(epoch.number() + 1)
        .save::<EpochModel>(base_dir)
        .unwrap();
}
