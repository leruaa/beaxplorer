use std::sync::Arc;

use lighthouse_types::{BeaconState, ChainSpec, Epoch, EthSpec, OwnedBeaconCommittee, Slot};
use store::SignedBeaconBlock;
use types::{
    attestation::{AttestationModel, AttestationModelsWithId},
    block::{BlockExtendedModelWithId, BlockModel, BlockModelWithId},
};

use super::block_state::BlockState;

#[derive(Debug, Clone)]
pub struct ConsolidatedBlock<E: EthSpec> {
    block: BlockState<E>,
    epoch: Epoch,
    slot: Slot,
    proposer_index: u64,
    committees: Vec<OwnedBeaconCommittee>,
}

impl<E: EthSpec> ConsolidatedBlock<E> {
    pub fn new(block: BlockState<E>, beacon_state: &BeaconState<E>, spec: &ChainSpec) -> Self {
        let slot = block.slot();
        let proposer_index = beacon_state.get_beacon_proposer_index(slot, spec).unwrap() as u64;
        let committees = beacon_state
            .get_beacon_committees_at_slot(slot)
            .unwrap()
            .into_iter()
            .map(|c| c.into_owned())
            .collect();

        ConsolidatedBlock {
            block,
            epoch: slot.epoch(E::slots_per_epoch()),
            slot,
            proposer_index,
            committees,
        }
    }

    pub fn get_attestations_count(&self) -> usize {
        match &self.block {
            BlockState::Proposed(block) => block.message().body().attestations().len(),
            BlockState::Missed(_) => 0,
            BlockState::Orphaned(_) => 0,
        }
    }

    pub fn get_deposits_count(&self) -> usize {
        match &self.block {
            BlockState::Proposed(block) => block.message().body().deposits().len(),
            BlockState::Missed(_) => 0,
            BlockState::Orphaned(_) => 0,
        }
    }

    pub fn get_voluntary_exits_count(&self) -> usize {
        match &self.block {
            BlockState::Proposed(block) => block.message().body().voluntary_exits().len(),
            BlockState::Missed(_) => 0,
            BlockState::Orphaned(_) => 0,
        }
    }

    pub fn get_proposer_slashings_count(&self) -> usize {
        match &self.block {
            BlockState::Proposed(block) => block.message().body().proposer_slashings().len(),
            BlockState::Missed(_) => 0,
            BlockState::Orphaned(_) => 0,
        }
    }

    pub fn get_attester_slashings_count(&self) -> usize {
        match &self.block {
            BlockState::Proposed(block) => block.message().body().attester_slashings().len(),
            BlockState::Missed(_) => 0,
            BlockState::Orphaned(_) => 0,
        }
    }
}

impl<E: EthSpec> From<&ConsolidatedBlock<E>> for BlockModelWithId {
    fn from(value: &ConsolidatedBlock<E>) -> Self {
        let model = BlockModel {
            epoch: value.epoch.as_u64(),
            proposer_slashings_count: value.get_proposer_slashings_count(),
            attester_slashings_count: value.get_attester_slashings_count(),
            attestations_count: value.get_attestations_count(),
            deposits_count: value.get_deposits_count(),
            voluntary_exits_count: value.get_voluntary_exits_count(),
            proposer: value.proposer_index,
            status: value.block.to_string(),
        };
        BlockModelWithId {
            id: value.slot.as_u64(),
            model,
        }
    }
}

impl<E: EthSpec> From<&ConsolidatedBlock<E>> for BlockExtendedModelWithId {
    fn from(value: &ConsolidatedBlock<E>) -> Self {
        BlockExtendedModelWithId {
            id: value.slot.as_u64(),
            model: (&value.block).into(),
        }
    }
}

impl<E: EthSpec> From<&ConsolidatedBlock<E>> for AttestationModelsWithId {
    fn from(value: &ConsolidatedBlock<E>) -> Self {
        let block: Option<Arc<SignedBeaconBlock<E>>> = (&value.block).into();

        let attestations = if let Some(block) = block {
            block
                .message()
                .body()
                .attestations()
                .iter()
                .map(AttestationModel::from)
                .collect::<Vec<AttestationModel>>()
        } else {
            vec![]
        };

        AttestationModelsWithId {
            id: value.slot.as_u64(),
            model: attestations,
        }
    }
}
