use std::{rc::Rc, sync::Arc};

use lighthouse_types::{BeaconState, BlindedPayload, ChainSpec, EthSpec, SignedBeaconBlock, Slot};
use parking_lot::RwLock;
use state_processing::{
    per_block_processing, per_epoch_processing::base::process_epoch, per_slot_processing,
    BlockReplayError, BlockSignatureStrategy, ConsensusContext, VerifyBlockRoot,
};
use task_executor::TaskExecutor;
use types::{
    block::{BlockExtendedModelWithId, BlockModelWithId, BlocksMeta},
    epoch::{EpochExtendedModelWithId, EpochModelWithId, EpochsMeta},
    persistable::Persistable,
};

use crate::{
    db::EpochToPersist,
    types::{
        block_state::BlockState, consolidated_block::ConsolidatedBlock,
        consolidated_epoch::ConsolidatedEpoch,
    },
};

pub struct PersistEpochWorker<E: EthSpec> {
    base_dir: String,
    beacon_state: Arc<RwLock<BeaconState<E>>>,
    spec: Arc<ChainSpec>,
}

impl<E: EthSpec> PersistEpochWorker<E> {
    pub fn new(base_dir: String, beacon_state: BeaconState<E>, spec: ChainSpec) -> Self {
        Self {
            base_dir,
            beacon_state: Arc::new(RwLock::new(beacon_state)),
            spec: Arc::new(spec),
        }
    }

    pub fn spawn(&self, executor: &TaskExecutor, epoch_to_persist: EpochToPersist<E>) {
        let base_dir = self.base_dir.clone();
        let beacon_state = self.beacon_state.clone();
        let spec = self.spec.clone();

        executor.spawn(
            async move {
                let epoch = epoch_to_persist.epoch;
                let mut blocks = epoch_to_persist.blocks.into_iter().collect::<Vec<_>>();

                blocks.sort_by(|(a, _), (b, _)| a.cmp(b));

                let b = blocks
                    .iter()
                    .filter_map(|(_, b)| match b {
                        BlockState::Proposed(b) => Some(b),
                        _ => None,
                    })
                    .map(|b| b.clone_as_blinded())
                    .collect::<Vec<_>>();

                let last_slot = epoch.end_slot(E::slots_per_epoch());

                apply_blocks(b, last_slot, &mut *beacon_state.write(), &spec).unwrap();

                let summary = process_epoch(&mut *beacon_state.write(), &spec).unwrap();

                let blocks = blocks
                    .into_iter()
                    .map(|(slot, block_status)| {
                        ConsolidatedBlock::new(
                            block_status,
                            slot,
                            epoch,
                            beacon_state
                                .read()
                                .get_beacon_proposer_index(slot, &spec)
                                .unwrap() as u64,
                        )
                    })
                    .collect::<Vec<_>>();

                let blocks = Rc::new(blocks);

                let block_models = blocks
                    .iter()
                    .map(BlockModelWithId::from)
                    .collect::<Vec<_>>();

                let extended_block_models = blocks
                    .iter()
                    .map(BlockExtendedModelWithId::from)
                    .collect::<Vec<_>>();

                let consolidated_epoch = ConsolidatedEpoch::new(
                    epoch,
                    blocks,
                    &summary,
                    beacon_state.read().balances().clone().into(),
                );

                let epoch_model = EpochModelWithId::from(&consolidated_epoch);

                let extended_epoch_model = EpochExtendedModelWithId::from(&consolidated_epoch);

                EpochsMeta::new(epoch.as_usize() + 1).persist(&base_dir);
                BlocksMeta::new(last_slot.as_usize() + 1).persist(&base_dir);

                epoch_model.persist(&base_dir);
                extended_epoch_model.persist(&base_dir);
                block_models.persist(&base_dir);
                extended_block_models.persist(&base_dir);
            },
            "persist epoch worker",
        )
    }
}

fn apply_blocks<E: EthSpec>(
    blocks: Vec<SignedBeaconBlock<E, BlindedPayload<E>>>,
    target_slot: Slot,
    beacon_state: &mut BeaconState<E>,
    spec: &ChainSpec,
) -> Result<(), BlockReplayError> {
    for (i, block) in blocks.iter().enumerate() {
        let slot = beacon_state.slot();
        // Allow one additional block at the start which is only used for its state root.
        if i == 0 && block.slot() <= slot {
            continue;
        }

        while slot < block.slot() {
            per_slot_processing(beacon_state, None, spec).map_err(BlockReplayError::from)?;
        }

        let mut consensus_context = ConsensusContext::new(block.slot());

        per_block_processing(
            beacon_state,
            block,
            BlockSignatureStrategy::NoVerification,
            VerifyBlockRoot::False,
            &mut consensus_context,
            spec,
        )
        .map_err(BlockReplayError::from)?;
    }

    while beacon_state.slot() < target_slot {
        per_slot_processing(beacon_state, None, spec).map_err(BlockReplayError::from)?;
    }

    Ok(())
}
