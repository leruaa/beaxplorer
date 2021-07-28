use std::{convert::TryInto, ops::Div};

use db::models::EpochModel;
use eth2::{lighthouse::GlobalValidatorInclusionData, types::ValidatorData};
use types::{Epoch, EthSpec};

use crate::errors::IndexerError;

use super::consolidated_block::ConsolidatedBlock;

#[derive(Debug)]
pub struct ConsolidatedEpoch<E: EthSpec> {
    pub epoch: Epoch,
    pub blocks: Vec<ConsolidatedBlock<E>>,
    pub validators: Vec<ValidatorData>,
    pub validator_inclusion: GlobalValidatorInclusionData,
}

impl<E: EthSpec> ConsolidatedEpoch<E> {
    pub fn as_model(&self) -> Result<EpochModel, IndexerError> {
        let epoch = self.epoch.as_u64().try_into()?;
        let total_validator_balance: i64 = self.get_total_validator_balance().try_into()?;
        let eligible_ether = self
            .validator_inclusion
            .previous_epoch_active_gwei
            .try_into()?;
        let voted_ether = self
            .validator_inclusion
            .previous_epoch_target_attesting_gwei
            .try_into()?;
        let global_participation_rate = (self
            .validator_inclusion
            .previous_epoch_target_attesting_gwei as f64)
            .div(self.validator_inclusion.previous_epoch_active_gwei as f64);

        let e = EpochModel {
            epoch,
            blocks_count: self.blocks.len() as i32,
            proposer_slashings_count: self.get_proposer_slashings_count() as i32,
            attester_slashings_count: self.get_attester_slashings_count() as i32,
            attestations_count: self.get_attestations_count() as i32,
            deposits_count: self.get_deposits_count() as i32,
            voluntary_exits_count: self.get_voluntary_exits_count() as i32,
            validators_count: self.validators.len() as i32,
            average_validator_balance: total_validator_balance.div(self.validators.len() as i64),
            total_validator_balance: total_validator_balance,
            finalized: Some(global_participation_rate >= 2f64 / 3f64),
            eligible_ether: Some(eligible_ether),
            global_participation_rate: Some(global_participation_rate),
            voted_ether: Some(voted_ether),
        };

        Ok(e)
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

    pub fn get_total_validator_balance(&self) -> u64 {
        self.validators
            .iter()
            .map(|v| v.validator.effective_balance)
            .sum()
    }
}
