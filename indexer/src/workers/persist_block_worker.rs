use lighthouse_types::EthSpec;
use task_executor::TaskExecutor;

use tracing::{debug, instrument};
use types::{
    attestation::AttestationModelsWithId,
    block::{BlockExtendedModelWithId, BlockModelWithId},
    persistable::Persistable,
};

use crate::types::consolidated_block::ConsolidatedBlock;

pub fn spawn_persist_block_worker<E: EthSpec>(
    base_dir: String,
    block: ConsolidatedBlock<E>,
    executor: &TaskExecutor,
) {
    executor.spawn(
        async move { persist_block(&base_dir, block) },
        "persist block worker",
    );
}

#[instrument(name = "BlockPersist", skip_all)]
fn persist_block<E: EthSpec>(base_dir: &str, block: ConsolidatedBlock<E>) {
    debug!(slot = %block.slot(), "Persisting block");

    BlockModelWithId::from(&block).persist(base_dir);
    BlockExtendedModelWithId::from(&block).persist(base_dir);
    AttestationModelsWithId::from(&block).persist(base_dir);
}
