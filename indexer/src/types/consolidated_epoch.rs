use std::ops::Div;
use std::rc::Rc;

use eth2::lighthouse::GlobalValidatorInclusionData;

use lighthouse_types::{Epoch, EthSpec};
use shared::utils::clock::Clock;
use state_processing::per_epoch_processing::EpochProcessingSummary;

use types::epoch::{EpochExtendedModel, EpochExtendedModelWithId, EpochModel, EpochModelWithId};

use super::consolidated_block::ConsolidatedBlock;

#[derive(Debug)]
pub struct ConsolidatedEpoch<E: EthSpec> {
    pub epoch: Epoch,
    pub blocks: Rc<Vec<ConsolidatedBlock<E>>>,
    pub validator_balances: Vec<u64>,
    pub validator_inclusion: GlobalValidatorInclusionData,
}

impl<E: EthSpec> ConsolidatedEpoch<E> {
    pub fn new(
        epoch: Epoch,
        blocks: Rc<Vec<ConsolidatedBlock<E>>>,
        summary: &EpochProcessingSummary<E>,
        validator_balances: Vec<u64>,
    ) -> Self {
        ConsolidatedEpoch::<E> {
            epoch,
            blocks,
            validator_balances,
            validator_inclusion: GlobalValidatorInclusionData {
                current_epoch_active_gwei: summary.current_epoch_total_active_balance(),
                previous_epoch_active_gwei: summary.previous_epoch_total_active_balance(),
                current_epoch_target_attesting_gwei: summary
                    .current_epoch_target_attesting_balance()
                    .unwrap_or(0),
                previous_epoch_target_attesting_gwei: summary
                    .previous_epoch_target_attesting_balance()
                    .unwrap_or(0),
                previous_epoch_head_attesting_gwei: summary
                    .previous_epoch_head_attesting_balance()
                    .unwrap_or(0),
            },
        }
    }

    pub fn get_attestations_count(&self) -> usize {
        self.blocks.iter().map(|b| b.get_attestations_count()).sum()
    }

    pub fn get_deposits_count(&self) -> usize {
        self.blocks.iter().map(|b| b.get_deposits_count()).sum()
    }

    pub fn get_voluntary_exits_count(&self) -> usize {
        self.blocks
            .iter()
            .map(|b| b.get_voluntary_exits_count())
            .sum()
    }

    pub fn get_proposer_slashings_count(&self) -> usize {
        self.blocks
            .iter()
            .map(|b| b.get_proposer_slashings_count())
            .sum()
    }

    pub fn get_attester_slashings_count(&self) -> usize {
        self.blocks
            .iter()
            .map(|b| b.get_attester_slashings_count())
            .sum()
    }

    pub fn get_total_validators_balance(&self) -> u64 {
        self.validator_balances.iter().sum()
    }
}

impl<E: EthSpec> From<&ConsolidatedEpoch<E>> for EpochModelWithId {
    fn from(value: &ConsolidatedEpoch<E>) -> Self {
        let start_slot = value.epoch.start_slot(E::slots_per_epoch());
        let spec = E::default_spec();
        let clock = Clock::new(spec);

        let eligible_ether = value.validator_inclusion.previous_epoch_active_gwei;
        let voted_ether = value
            .validator_inclusion
            .previous_epoch_target_attesting_gwei;

        let model = EpochModel {
            timestamp: clock.timestamp(start_slot).unwrap_or(0),
            proposer_slashings_count: value.get_proposer_slashings_count(),
            attester_slashings_count: value.get_attester_slashings_count(),
            attestations_count: value.get_attestations_count(),
            deposits_count: value.get_deposits_count(),
            eligible_ether,
            voted_ether,
        };

        EpochModelWithId {
            id: value.epoch.as_u64(),
            model,
        }
    }
}

impl<E: EthSpec> From<&ConsolidatedEpoch<E>> for EpochExtendedModelWithId {
    fn from(value: &ConsolidatedEpoch<E>) -> Self {
        let model = EpochExtendedModel {
            voluntary_exits_count: value.get_voluntary_exits_count(),
            validators_count: value.validator_balances.len(),
            average_validator_balance: value
                .get_total_validators_balance()
                .div(value.validator_balances.len() as u64),
            total_validator_balance: value.get_total_validators_balance(),
        };

        EpochExtendedModelWithId {
            id: value.epoch.as_u64(),
            model,
        }
    }
}
