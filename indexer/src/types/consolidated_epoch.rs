use std::convert::TryInto;

use db::models::EpochModel;
use types::{Epoch, EthSpec, Validator};

use crate::errors::IndexerError;

use super::consolidated_block::ConsolidatedBlock;

#[derive(Debug)]
pub struct ConsolidatedEpoch<E: EthSpec> {
    pub epoch: Epoch,
    pub blocks: Vec<ConsolidatedBlock<E>>,
    pub validators: Vec<Validator>,
}

impl<E: EthSpec> ConsolidatedEpoch<E> {
    pub fn new(epoch: Epoch) -> Self {
        ConsolidatedEpoch {
            epoch,
            blocks: Vec::new(),
            validators: Vec::new(),
        }
    }

    pub fn as_model(&self) -> Result<EpochModel, IndexerError> {
        let epoch_as_i64 = self.epoch.as_u64().try_into()?;
        let total_validator_balance_as_i64: i64 = self.get_total_validator_balance().try_into()?;

        let epoch = EpochModel {
            epoch: epoch_as_i64,
            blocks_count: self.blocks.len() as i32,
            proposer_slashings_count: self.get_proposer_slashings_count() as i32,
            attester_slashings_count: self.get_attester_slashings_count() as i32,
            attestations_count: self.get_attestations_count() as i32,
            deposits_count: self.get_deposits_count() as i32,
            voluntary_exits_count: self.get_voluntary_exits_count() as i32,
            validators_count: self.validators.len() as i32,
            average_validator_balance: total_validator_balance_as_i64
                .div_euclid(self.validators.len() as i64),
            total_validator_balance: total_validator_balance_as_i64,
            finalized: Some(true),
            eligible_ether: None,
            global_participation_rate: None,
            voted_ether: None,
        };

        Ok(epoch)
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
        self.validators.iter().map(|v| v.effective_balance).sum()
    }
}
