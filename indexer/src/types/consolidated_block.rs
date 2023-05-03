use std::sync::Arc;

use lighthouse_types::{Attestation, DepositData, Epoch, EthSpec, OwnedBeaconCommittee, Slot};
use shared::utils::clock::Clock;
use store::SignedBeaconBlock;
use types::{
    attestation::{AttestationModel, AttestationModelsWithId},
    block::{BlockExtendedModelWithId, BlockModel, BlockModelWithId},
    block_root::{BlockRootModel, BlockRootModelWithId},
    committee::{CommitteeModel, CommitteeModelsWithId},
    deposit::DepositModel,
};

use super::block_state::BlockState;

#[derive(Debug, Clone)]
pub struct ConsolidatedBlock<E: EthSpec> {
    block: BlockState<E>,
    epoch: Epoch,
    slot: Slot,
    proposer_index: u64,
    committees: Vec<OwnedBeaconCommittee>,
    pub deposits: Vec<DepositData>,
}

impl<E: EthSpec> ConsolidatedBlock<E> {
    pub fn new(
        block: BlockState<E>,
        proposer_index: u64,
        committees: Vec<OwnedBeaconCommittee>,
        deposits: Vec<DepositData>,
    ) -> Self {
        let slot = block.slot();

        ConsolidatedBlock {
            block,
            epoch: slot.epoch(E::slots_per_epoch()),
            slot,
            proposer_index,
            committees,
            deposits,
        }
    }

    pub fn slot(&self) -> Slot {
        self.slot
    }

    pub fn attestations(&self) -> Vec<Attestation<E>> {
        match &self.block {
            BlockState::Proposed(block) => block.message().body().attestations().to_vec(),
            _ => vec![],
        }
    }

    pub fn get_attestations_count(&self) -> usize {
        match &self.block {
            BlockState::Proposed(block) => block.message().body().attestations().len(),
            _ => 0,
        }
    }

    pub fn get_deposits_count(&self) -> usize {
        match &self.block {
            BlockState::Proposed(block) => block.message().body().deposits().len(),
            _ => 0,
        }
    }

    pub fn get_voluntary_exits_count(&self) -> usize {
        match &self.block {
            BlockState::Proposed(block) => block.message().body().voluntary_exits().len(),
            _ => 0,
        }
    }

    pub fn get_proposer_slashings_count(&self) -> usize {
        match &self.block {
            BlockState::Proposed(block) => block.message().body().proposer_slashings().len(),
            _ => 0,
        }
    }

    pub fn get_attester_slashings_count(&self) -> usize {
        match &self.block {
            BlockState::Proposed(block) => block.message().body().attester_slashings().len(),
            _ => 0,
        }
    }
}

impl<E: EthSpec> From<&ConsolidatedBlock<E>> for BlockModelWithId {
    fn from(value: &ConsolidatedBlock<E>) -> Self {
        let spec = E::default_spec();
        let clock = Clock::new(spec);

        let model = BlockModel {
            epoch: value.epoch.as_u64(),
            timestamp: clock.timestamp(value.slot).unwrap_or(0),
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

impl<E: EthSpec> From<&ConsolidatedBlock<E>> for Option<BlockRootModelWithId> {
    fn from(value: &ConsolidatedBlock<E>) -> Self {
        value.block.root().map(|root| BlockRootModelWithId {
            id: format!("{:?}", root),
            model: BlockRootModel {
                slot: value.slot.as_u64(),
            },
        })
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

impl<E: EthSpec> From<&ConsolidatedBlock<E>> for CommitteeModelsWithId {
    fn from(value: &ConsolidatedBlock<E>) -> Self {
        let committees = value.committees.iter().map(CommitteeModel::from).collect();

        CommitteeModelsWithId {
            id: value.slot.as_u64(),
            model: committees,
        }
    }
}

impl<E: EthSpec> From<&ConsolidatedBlock<E>> for Vec<DepositModel> {
    fn from(value: &ConsolidatedBlock<E>) -> Self {
        value
            .deposits
            .iter()
            .map(|d| DepositModel {
                slot: value.slot.as_u64(),
                public_key: d.pubkey.to_string(),
                withdrawal_credentials: d.withdrawal_credentials.as_bytes().to_vec(),
                amount: d.amount,
                signature: d.signature.to_string(),
            })
            .collect()
    }
}
