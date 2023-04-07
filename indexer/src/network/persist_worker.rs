use std::sync::Arc;

use lighthouse_types::{BeaconState, ChainSpec, EthSpec, RelativeEpoch};
use state_processing::{
    per_block_processing, per_slot_processing, BlockReplayError, BlockSignatureStrategy,
    ConsensusContext, VerifyBlockRoot,
};
use task_executor::TaskExecutor;
use tokio::sync::{
    mpsc::{self, UnboundedSender},
    watch::Receiver,
};
use tracing::{debug, error, info};
use types::{
    attestation::AttestationModelsWithId,
    block::{BlockExtendedModelWithId, BlockModelWithId},
    persistable::Persistable,
};

use crate::{
    beacon_chain::beacon_context::BeaconContext,
    types::{block_state::BlockState, consolidated_block::ConsolidatedBlock},
};

pub fn spawn_persist_worker<E: EthSpec>(
    executor: &TaskExecutor,
    base_dir: String,
    beacon_context: Arc<BeaconContext<E>>,
    mut shutdown_trigger: Receiver<()>,
) -> UnboundedSender<BlockState<E>> {
    let (work_send, mut work_recv) = mpsc::unbounded_channel();

    let mut beacon_state = beacon_context.genesis_state.clone();

    beacon_state
        .build_committee_cache(RelativeEpoch::Current, &beacon_context.spec)
        .unwrap();

    executor.spawn(
        async move {
            loop {
                tokio::select! {
                    Some(block) = work_recv.recv() => {
                        handle(block, &base_dir, &mut beacon_state, &beacon_context.spec);
                    }

                    _ = shutdown_trigger.changed() => {
                        info!("Shutting down persist worker...");
                        return;
                    }
                }
            }
        },
        "persist epoch worker",
    );

    work_send
}

fn handle<E: EthSpec>(
    block: BlockState<E>,
    base_dir: &str,
    beacon_state: &mut BeaconState<E>,
    spec: &ChainSpec,
) {
    if beacon_state.slot().as_u64() == 0 || block.slot() > beacon_state.slot() {
        debug!(slot = %block.slot(), "Persist block");
        match handle_block(&block, base_dir, beacon_state.clone(), spec) {
            Ok(new_state) => {
                *beacon_state = new_state;
            }
            Err(err) => error!(slot = %block.slot(), "{err:?}"),
        }
    }
}

fn handle_block<E: EthSpec>(
    block: &BlockState<E>,
    base_dir: &str,
    mut beacon_state: BeaconState<E>,
    spec: &ChainSpec,
) -> Result<BeaconState<E>, BlockReplayError> {
    let summary = match block {
        BlockState::Proposed(beacon_block) => {
            let summary = per_slot_processing(&mut beacon_state, None, spec)
                .map_err(BlockReplayError::from)?;
            let mut consensus_context = ConsensusContext::new(beacon_block.slot());

            if block.slot() > 0 {
                per_block_processing(
                    &mut beacon_state,
                    beacon_block.as_ref(),
                    BlockSignatureStrategy::NoVerification,
                    VerifyBlockRoot::False,
                    &mut consensus_context,
                    spec,
                )
                .map_err(BlockReplayError::from)?;
            }

            persist_block(block.clone(), base_dir, &beacon_state, spec);
            summary
        }
        BlockState::Missed(_) => {
            let summary = per_slot_processing(&mut beacon_state, None, spec)
                .map_err(BlockReplayError::from)?;

            persist_block(block.clone(), base_dir, &beacon_state, spec);
            summary
        }
        BlockState::Orphaned(_) => {
            persist_block(block.clone(), base_dir, &beacon_state, spec);
            None
        }
    };

    if summary.is_some() {
        info!(epoch = %beacon_state.previous_epoch(), "Epoch completed")
    }

    Ok(beacon_state)
}

fn persist_block<E: EthSpec>(
    block: BlockState<E>,
    base_dir: &str,
    beacon_state: &BeaconState<E>,
    spec: &ChainSpec,
) {
    let block = ConsolidatedBlock::new(block, beacon_state, spec);

    BlockModelWithId::from(&block).persist(base_dir);
    BlockExtendedModelWithId::from(&block).persist(base_dir);
    AttestationModelsWithId::from(&block).persist(base_dir);
}
