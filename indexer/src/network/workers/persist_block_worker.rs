use std::sync::Arc;

use lighthouse_types::{BeaconState, ChainSpec, EthSpec};
use parking_lot::RwLock;
use task_executor::TaskExecutor;
use tokio::sync::{
    mpsc::{self, UnboundedSender},
    watch::Receiver,
};
use tracing::info;
use types::{
    attestation::AttestationModelsWithId,
    block::{BlockExtendedModelWithId, BlockModelWithId},
    path::FromPath,
    persistable::Persistable,
};

use crate::types::{block_state::BlockState, consolidated_block::ConsolidatedBlock};

pub struct PersistBlockWorker<E: EthSpec> {
    work_send: UnboundedSender<BlockState<E>>,
}

impl<E: EthSpec> PersistBlockWorker<E> {
    pub fn new(
        executor: &TaskExecutor,
        base_dir: String,
        beacon_state: Arc<RwLock<BeaconState<E>>>,
        spec: ChainSpec,
        mut shutdown_trigger: Receiver<()>,
    ) -> Self {
        let (work_send, mut work_recv) = mpsc::unbounded_channel();

        executor.spawn(
            async move {
                loop {
                    tokio::select! {
                        Some(block_state) = work_recv.recv() => {
                            persist_block(&base_dir, block_state, &mut beacon_state.write(), &spec);
                        }

                        _ = shutdown_trigger.changed() => {
                            info!("Shutting down block worker...");
                            return;
                        }
                    }
                }
            },
            "generic persist worker",
        );

        Self { work_send }
    }

    pub fn work(&self, block_state: BlockState<E>) {
        self.work_send.send(block_state).unwrap();
    }
}

fn persist_block<E: EthSpec>(
    base_dir: &str,
    block_state: BlockState<E>,
    beacon_state: &mut BeaconState<E>,
    spec: &ChainSpec,
) {
    let slot = block_state.slot();

    let block = ConsolidatedBlock::new(block_state, beacon_state, spec);

    BlockModelWithId::from(&block).persist(base_dir);
    BlockExtendedModelWithId::from(&block).persist(base_dir);
    AttestationModelsWithId::from(&block).persist(base_dir);
}
