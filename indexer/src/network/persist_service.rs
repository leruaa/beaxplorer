use std::{
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
    sync::Arc,
};

use lighthouse_types::{
    BeaconState, BlindedPayload, ChainSpec, Epoch, EthSpec, SignedBeaconBlock, Slot,
};
use parking_lot::RwLock;
use slog::{info, warn, Logger};
use state_processing::{
    per_block_processing, per_epoch_processing::process_epoch, per_slot_processing,
    BlockReplayError, BlockSignatureStrategy, ConsensusContext, VerifyBlockRoot,
};
use types::{
    block::{BlockExtendedModelWithId, BlockModelWithId, BlocksMeta},
    block_request::BlockRequestsMeta,
    epoch::{EpochExtendedModelWithId, EpochModelWithId, EpochsMeta},
    path::FromPath,
    persistable::Persistable,
};

use crate::{
    beacon_chain::beacon_context::BeaconContext,
    direct_indexer::BlockMessage,
    network::block_by_root_requests::BlockByRootRequests,
    types::{
        consolidated_block::{BlockStatus, ConsolidatedBlock},
        consolidated_epoch::ConsolidatedEpoch,
    },
};

pub struct PersistService<E: EthSpec> {
    base_dir: String,
    current_epoch: Epoch,
    blocks_by_epoch: HashMap<Epoch, HashMap<Slot, BlockMessage<E>>>,
    spec: ChainSpec,
    beacon_state: BeaconState<E>,
    block_by_root_requests: Arc<RwLock<BlockByRootRequests>>,
    log: Logger,
}

impl<E: EthSpec> PersistService<E> {
    pub fn new(
        base_dir: String,
        beacon_context: Arc<BeaconContext<E>>,
        block_by_root_requests: Arc<RwLock<BlockByRootRequests>>,
        log: Logger,
    ) -> Self {
        Self {
            base_dir,
            current_epoch: Epoch::new(0),
            blocks_by_epoch: HashMap::new(),
            spec: beacon_context.spec.clone(),
            beacon_state: beacon_context.genesis_state.clone(),
            block_by_root_requests,
            log,
        }
    }

    pub fn handle_block_message(&mut self, block_message: BlockMessage<E>) {
        let slot = match &block_message {
            BlockMessage::Proposed(block) => block.message().slot(),
            BlockMessage::Orphaned(block) => block.message().slot(),
            BlockMessage::Missed(slot) => *slot,
        };

        let epoch = slot.epoch(E::slots_per_epoch());

        if epoch >= self.current_epoch {
            let blocks_by_slot = self
                .blocks_by_epoch
                .entry(epoch)
                .or_insert_with(HashMap::new);

            match blocks_by_slot.entry(slot) {
                Entry::Occupied(mut e) => {
                    if let BlockMessage::Missed(_) = e.get() {
                        e.insert(block_message);
                    }
                }
                Entry::Vacant(e) => {
                    e.insert(block_message);
                }
            };

            if epoch == self.current_epoch && blocks_by_slot.len() as u64 == E::slots_per_epoch() {
                if let Some(blocks_by_slot) = self.blocks_by_epoch.remove(&epoch) {
                    let mut blocks_by_slot = blocks_by_slot
                        .iter()
                        .map(|(s, b)| (s, BlockStatus::from(b)))
                        .collect::<Vec<_>>();

                    blocks_by_slot.sort_by(|(a, _), (b, _)| a.cmp(b));

                    self.persist_epoch(&epoch, blocks_by_slot);
                    self.current_epoch = Epoch::new(self.current_epoch.as_u64() + 1);
                    self.persist_block_roots();
                }
            }
        } else if let BlockMessage::Orphaned(block) = block_message {
            // Persist orphaned even if we get them too late
            self.persist_existing_block(BlockStatus::Orphaned(block), &slot, &epoch);
        }
    }

    fn persist_epoch(&mut self, epoch: &Epoch, blocks: Vec<(&Slot, BlockStatus<E>)>) {
        info!(self.log, "Persisting epoch {epoch}");

        let b = blocks
            .iter()
            .filter_map(|(_, b)| match b {
                BlockStatus::Proposed(b) => Some(b),
                _ => None,
            })
            .map(|b| b.clone_as_blinded())
            .collect::<Vec<_>>();

        let last_slot = epoch.end_slot(E::slots_per_epoch());

        self.apply_blocks(b, last_slot).unwrap();

        let summary = process_epoch(&mut self.beacon_state.clone(), &self.spec).unwrap();

        let blocks = blocks
            .into_iter()
            .map(|(slot, block_status)| {
                ConsolidatedBlock::new(
                    block_status,
                    *slot,
                    *epoch,
                    self.beacon_state
                        .get_beacon_proposer_index(*slot, &self.spec)
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
            *epoch,
            blocks,
            summary,
            self.beacon_state.balances().clone().into(),
        );

        let epoch_model = EpochModelWithId::from(&consolidated_epoch);

        let extended_epoch_model = EpochExtendedModelWithId::from(&consolidated_epoch);

        EpochsMeta::new(epoch.as_usize() + 1).persist(&self.base_dir);
        BlocksMeta::new(last_slot.as_usize() + 1).persist(&self.base_dir);

        epoch_model.persist(&self.base_dir);
        extended_epoch_model.persist(&self.base_dir);
        block_models.persist(&self.base_dir);
        extended_block_models.persist(&self.base_dir);
    }

    fn persist_existing_block(&self, block_status: BlockStatus<E>, slot: &Slot, epoch: &Epoch) {
        let block_model = BlockModelWithId::from_path(&self.base_dir, &slot.as_u64());

        if block_model.status == "Missed" {
            let block = ConsolidatedBlock::new(block_status, *slot, *epoch, block_model.proposer);

            BlockModelWithId::from(&block).persist(&self.base_dir);
            BlockExtendedModelWithId::from(&block).persist(&self.base_dir);
        } else {
            warn!(self.log, "Block {} not persisted", slot);
        }
    }

    fn persist_block_roots(&self) {
        let block_by_root_requests_state = self.block_by_root_requests.read();

        info!(self.log, "Persisting pending block roots");
        block_by_root_requests_state.persist(&self.base_dir);
        BlockRequestsMeta::new(block_by_root_requests_state.count()).persist(&self.base_dir);
    }

    fn apply_blocks(
        &mut self,
        blocks: Vec<SignedBeaconBlock<E, BlindedPayload<E>>>,
        target_slot: Slot,
    ) -> Result<(), BlockReplayError> {
        for (i, block) in blocks.iter().enumerate() {
            // Allow one additional block at the start which is only used for its state root.
            if i == 0 && block.slot() <= self.beacon_state.slot() {
                continue;
            }

            while self.beacon_state.slot() < block.slot() {
                per_slot_processing(&mut self.beacon_state, None, &self.spec)
                    .map_err(BlockReplayError::from)?;
            }

            let mut consensus_context = ConsensusContext::new(block.slot());

            per_block_processing(
                &mut self.beacon_state,
                block,
                BlockSignatureStrategy::NoVerification,
                VerifyBlockRoot::False,
                &mut consensus_context,
                &self.spec,
            )
            .map_err(BlockReplayError::from)?;
        }

        while self.beacon_state.slot() < target_slot {
            per_slot_processing(&mut self.beacon_state, None, &self.spec)
                .map_err(BlockReplayError::from)?;
        }

        Ok(())
    }
}
