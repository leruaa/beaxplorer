use std::{collections::HashMap, rc::Rc, sync::Arc};

use lighthouse_types::{
    BeaconState, BlindedPayload, ChainSpec, Epoch, EthSpec, SignedBeaconBlock, Slot,
};

use parking_lot::RwLock;
use state_processing::{
    per_block_processing, per_epoch_processing::base::process_epoch, per_slot_processing,
    BlockReplayError, BlockSignatureStrategy, ConsensusContext, VerifyBlockRoot,
};
use task_executor::TaskExecutor;
use tokio::sync::{
    mpsc::{self, UnboundedSender},
    watch::Receiver,
};
use tracing::{error, info, instrument};
use types::{
    attestation::AttestationModelsWithId,
    block::{BlockExtendedModelWithId, BlockModelWithId, BlocksMeta},
    epoch::{EpochExtendedModelWithId, EpochModelWithId, EpochsMeta},
    persistable::Persistable,
};

use crate::types::{
    block_state::BlockState, consolidated_block::ConsolidatedBlock,
    consolidated_epoch::ConsolidatedEpoch,
};

pub struct PersistEpochWorker<E: EthSpec> {
    work_send: UnboundedSender<(Epoch, HashMap<Slot, BlockState<E>>)>,
}

impl<E: EthSpec> PersistEpochWorker<E> {
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
                        Some((epoch, blocks)) = work_recv.recv() => {
                            handle_work(epoch, blocks, &base_dir, &mut beacon_state.write(), &spec);
                        }

                        _ = shutdown_trigger.changed() => {
                            info!("Shutting down epoch worker...");
                            return;
                        }
                    }
                }
            },
            "persist epoch worker",
        );

        Self { work_send }
    }

    pub fn work(&self, epoch: Epoch, blocks: HashMap<Slot, BlockState<E>>) {
        self.work_send.send((epoch, blocks)).unwrap();
    }
}

#[instrument(name = "EpochPersist", skip_all)]
fn handle_work<E: EthSpec>(
    epoch: Epoch,
    blocks: HashMap<Slot, BlockState<E>>,
    base_dir: &str,
    beacon_state: &mut BeaconState<E>,
    spec: &ChainSpec,
) {
    info!(%epoch);

    let mut blocks = blocks.into_iter().collect::<Vec<_>>();

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

    match apply_blocks(b, last_slot, beacon_state, spec) {
        Ok(_) => match process_epoch(&mut beacon_state.clone(), spec) {
            Ok(summary) => {
                let blocks = blocks
                    .into_iter()
                    .map(|(slot, block_status)| {
                        ConsolidatedBlock::new(
                            block_status,
                            beacon_state.get_beacon_proposer_index(slot, spec).unwrap() as u64,
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
                    blocks.clone(),
                    &summary,
                    beacon_state.balances().clone().into(),
                );

                let epoch_model = EpochModelWithId::from(&consolidated_epoch);
                let extended_epoch_model = EpochExtendedModelWithId::from(&consolidated_epoch);

                let attestation_models = blocks
                    .iter()
                    .map(AttestationModelsWithId::from)
                    .collect::<Vec<_>>();

                EpochsMeta::new(epoch.as_usize() + 1).persist(base_dir);
                BlocksMeta::new(last_slot.as_usize() + 1).persist(base_dir);

                epoch_model.persist(base_dir);
                extended_epoch_model.persist(base_dir);
                block_models.persist(base_dir);
                extended_block_models.persist(base_dir);
                attestation_models.persist(base_dir);
            }
            Err(err) => error!(?err),
        },
        Err(err) => error!(?err),
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

        while beacon_state.slot() < block.slot() {
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
