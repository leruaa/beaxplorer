use lighthouse_types::{Epoch, EthSpec, Slot};
use task_executor::TaskExecutor;
use types::{
    block::{BlockExtendedModelWithId, BlockModelWithId},
    path::FromPath,
    persistable::Persistable,
};

use crate::types::{block_state::BlockState, consolidated_block::ConsolidatedBlock};

pub struct PersistExistingBlockWorker {
    base_dir: String,
}

impl PersistExistingBlockWorker {
    pub fn new(base_dir: String) -> Self {
        Self { base_dir }
    }

    pub fn spawn<E: EthSpec>(&self, executor: &TaskExecutor, block_state: BlockState<E>) {
        let base_dir = self.base_dir.clone();

        if let BlockState::Orphaned(block) = &block_state {
            let slot = block.slot();
            let epoch = slot.epoch(E::slots_per_epoch());

            executor.spawn(
                async move {
                    persist_existing_block(&base_dir, block_state, &slot, &epoch);
                },
                "generic persist worker",
            )
        }
    }
}

fn persist_existing_block<E: EthSpec>(
    base_dir: &str,
    block_state: BlockState<E>,
    slot: &Slot,
    epoch: &Epoch,
) {
    let block_model = BlockModelWithId::from_path(base_dir, &slot.as_u64());

    if block_model.status == "Missed" {
        let block = ConsolidatedBlock::new(block_state, *slot, *epoch, block_model.proposer);

        BlockModelWithId::from(&block).persist(base_dir);
        BlockExtendedModelWithId::from(&block).persist(base_dir);
    }
}
