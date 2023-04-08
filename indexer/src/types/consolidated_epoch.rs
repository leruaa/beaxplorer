use std::{fmt::Display, marker::PhantomData, ops::Div};

use eth2::lighthouse::GlobalValidatorInclusionData;

use lighthouse_types::{Epoch, EthSpec};
use shared::utils::clock::Clock;
use state_processing::per_epoch_processing::EpochProcessingSummary;

use types::epoch::{EpochExtendedModel, EpochExtendedModelWithId, EpochModel, EpochModelWithId};

#[derive(Debug)]
pub struct ConsolidatedEpoch<E: EthSpec> {
    epoch: Epoch,
    aggregated_data: AggregatedEpochData,
    validator_balances: Vec<u64>,
    validator_inclusion: GlobalValidatorInclusionData,
    phantom: PhantomData<E>,
}

impl<E: EthSpec> ConsolidatedEpoch<E> {
    pub fn new(
        epoch: Epoch,
        aggregated_data: AggregatedEpochData,
        summary: &EpochProcessingSummary<E>,
        validator_balances: Vec<u64>,
    ) -> Self {
        ConsolidatedEpoch::<E> {
            epoch,
            aggregated_data,
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
            phantom: PhantomData::default(),
        }
    }

    pub fn number(&self) -> usize {
        self.epoch.as_usize()
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
            proposer_slashings_count: value.aggregated_data.proposer_slashings_count,
            attester_slashings_count: value.aggregated_data.attester_slashings_count,
            attestations_count: value.aggregated_data.attestations_count,
            deposits_count: value.aggregated_data.deposits_count,
            eligible_ether,
            voted_ether,
        };

        EpochModelWithId {
            id: value.epoch.as_u64(),
            model,
        }
    }
}

impl<E: EthSpec> Display for ConsolidatedEpoch<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.epoch)
    }
}

impl<E: EthSpec> From<&ConsolidatedEpoch<E>> for EpochExtendedModelWithId {
    fn from(value: &ConsolidatedEpoch<E>) -> Self {
        let model = EpochExtendedModel {
            voluntary_exits_count: value.aggregated_data.voluntary_exits_count,
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

#[derive(Debug, Clone, Default)]
pub struct AggregatedEpochData {
    pub attestations_count: usize,
    pub deposits_count: usize,
    pub voluntary_exits_count: usize,
    pub proposer_slashings_count: usize,
    pub attester_slashings_count: usize,
}
