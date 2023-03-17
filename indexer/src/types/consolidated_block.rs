use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

use lighthouse_types::{Epoch, EthSpec, Slot};
use store::SignedBeaconBlock;
use types::{
    attestation::{AttestationModel, AttestationModelsWithId},
    block::{BlockExtendedModel, BlockExtendedModelWithId, BlockModel, BlockModelWithId},
};

use crate::network::persist_service::BlockMessage;

#[derive(Debug, Clone)]
pub enum BlockStatus<E: EthSpec> {
    Proposed(Arc<SignedBeaconBlock<E>>),
    Missed,
    Orphaned(Arc<SignedBeaconBlock<E>>),
}

impl<E: EthSpec> Display for BlockStatus<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockStatus::Proposed(_) => write!(f, "Proposed"),
            BlockStatus::Missed => write!(f, "Missed"),
            BlockStatus::Orphaned(_) => write!(f, "Orphaned"),
        }
    }
}

impl<E: EthSpec> From<&BlockMessage<E>> for BlockStatus<E> {
    fn from(value: &BlockMessage<E>) -> Self {
        match value {
            BlockMessage::Proposed(block) => BlockStatus::Proposed(block.clone()),
            BlockMessage::Orphaned(block) => BlockStatus::Orphaned(block.clone()),
            BlockMessage::Missed(_) => BlockStatus::Missed,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConsolidatedBlock<E: EthSpec> {
    block: BlockStatus<E>,
    epoch: Epoch,
    slot: Slot,
    proposer_index: u64,
}

impl<E: EthSpec> ConsolidatedBlock<E> {
    fn block(&self) -> Option<Arc<SignedBeaconBlock<E>>> {
        match &self.block {
            BlockStatus::Proposed(block) => Some(block.clone()),
            BlockStatus::Missed => None,
            BlockStatus::Orphaned(block) => Some(block.clone()),
        }
    }

    pub fn new(block: BlockStatus<E>, slot: Slot, epoch: Epoch, proposer_index: u64) -> Self {
        ConsolidatedBlock {
            block,
            epoch,
            slot,
            proposer_index,
        }
    }

    pub fn get_attestations_count(&self) -> usize {
        match &self.block {
            BlockStatus::Proposed(block) => block.message().body().attestations().len(),
            BlockStatus::Missed => 0,
            BlockStatus::Orphaned(_) => 0,
        }
    }

    pub fn get_deposits_count(&self) -> usize {
        match &self.block {
            BlockStatus::Proposed(block) => block.message().body().deposits().len(),
            BlockStatus::Missed => 0,
            BlockStatus::Orphaned(_) => 0,
        }
    }

    pub fn get_voluntary_exits_count(&self) -> usize {
        match &self.block {
            BlockStatus::Proposed(block) => block.message().body().voluntary_exits().len(),
            BlockStatus::Missed => 0,
            BlockStatus::Orphaned(_) => 0,
        }
    }

    pub fn get_proposer_slashings_count(&self) -> usize {
        match &self.block {
            BlockStatus::Proposed(block) => block.message().body().proposer_slashings().len(),
            BlockStatus::Missed => 0,
            BlockStatus::Orphaned(_) => 0,
        }
    }

    pub fn get_attester_slashings_count(&self) -> usize {
        match &self.block {
            BlockStatus::Proposed(block) => block.message().body().attester_slashings().len(),
            BlockStatus::Missed => 0,
            BlockStatus::Orphaned(_) => 0,
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
        let model = value.block().map(|block| BlockExtendedModel {
            block_root: block.canonical_root().as_bytes().to_vec(),
            parent_root: block.message().parent_root().as_bytes().to_vec(),
            state_root: block.message().state_root().as_bytes().to_vec(),
            randao_reveal: block
                .message()
                .body()
                .randao_reveal()
                .to_string()
                .as_bytes()
                .to_vec(),
            signature: block.signature().to_string().as_bytes().to_vec(),
            graffiti: block
                .message()
                .body()
                .graffiti()
                .to_string()
                .as_bytes()
                .to_vec(),
            graffiti_text: block.message().body().graffiti().to_string(),
            votes_count: 0,
            eth1data_deposit_root: block
                .message()
                .body()
                .eth1_data()
                .deposit_root
                .as_bytes()
                .to_vec(),
            eth1data_deposit_count: block.message().body().eth1_data().deposit_count,
            eth1data_block_hash: block
                .message()
                .body()
                .eth1_data()
                .block_hash
                .as_bytes()
                .to_vec(),
        });

        BlockExtendedModelWithId {
            id: value.slot.as_u64(),
            model,
        }
    }
}

impl<E: EthSpec> From<&ConsolidatedBlock<E>> for AttestationModelsWithId {
    fn from(value: &ConsolidatedBlock<E>) -> Self {
        let attestations = if let Some(block) = value.block() {
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
