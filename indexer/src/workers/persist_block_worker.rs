use lighthouse_types::EthSpec;
use task_executor::TaskExecutor;

use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedSender},
    watch::Receiver,
};
use tracing::{debug, info, instrument};
use types::{
    attestation::AttestationModelsWithId,
    block::{BlockExtendedModelWithId, BlockModelWithId, BlocksMeta},
    block_root::BlockRootModelWithId,
    committee::CommitteeModelsWithId,
    persistable::Persistable,
};

use crate::types::consolidated_block::ConsolidatedBlock;

pub fn spawn_persist_block_worker<E: EthSpec>(
    base_dir: String,
    mut shutdown_trigger: Receiver<()>,
    executor: &TaskExecutor,
) -> UnboundedSender<ConsolidatedBlock<E>> {
    let (new_block_send, mut new_block_recv) = unbounded_channel();

    executor.spawn(
        async move {
            loop {
                tokio::select! {
                    Some(block) = new_block_recv.recv() => {
                        persist_block::<E>(&base_dir, block);
                    }

                    _ = shutdown_trigger.changed() => {
                        info!("Shutting down blocks worker...");
                        return;
                    }
                }
            }
        },
        "persist block worker",
    );

    new_block_send
}

#[instrument(name = "BlockPersist", skip_all)]
fn persist_block<E: EthSpec>(base_dir: &str, block: ConsolidatedBlock<E>) {
    debug!(slot = %block.slot(), "Persisting block");

    BlockModelWithId::from(&block).persist(base_dir);
    BlockExtendedModelWithId::from(&block).persist(base_dir);
    AttestationModelsWithId::from(&block).persist(base_dir);
    CommitteeModelsWithId::from(&block).persist(base_dir);
    BlockRootModelWithId::from(&block).persist(base_dir);

    BlocksMeta::new(block.slot().as_usize() + 1).persist(base_dir);
}
