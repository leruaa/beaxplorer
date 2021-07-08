use std::{collections::HashMap, convert::{TryInto}};

use db::models::EpochModel;
use types::{Epoch, EthSpec, Slot, Validator};

use crate::errors::IndexerError;

use super::consolidated_block::ConsolidatedBlock;

#[derive(Debug)]
pub struct ConsolidatedEpoch<E: EthSpec> {
    pub epoch: Epoch,
    pub blocks: HashMap<Slot, ConsolidatedBlock<E>>,
    pub validators: Vec<Validator>,
}

impl<E: EthSpec> ConsolidatedEpoch<E> {
    pub fn new(epoch: Epoch) -> Self {
        ConsolidatedEpoch {
            epoch,
            blocks: HashMap::new(),
            validators: Vec::new(),
        }
    }

    pub fn as_model(&self) -> Result<EpochModel, IndexerError> {
        let epoch_as_i64 = self.epoch
            .as_u64()
            .try_into()
            .map_err(|source| IndexerError::EpochCastingFailed { source } )?;

        let epoch = EpochModel {
            epoch: epoch_as_i64,
            blocks_count: self.blocks.len() as i32,
            proposer_slashings_count: 0,
            attester_slashings_count: 0,
            attestations_count: 0,
            deposits_count: 0,
            voluntary_exits_count: 0,
            validators_count: self.validators.len() as i32,
            average_validator_balance: 0,
            total_validator_balance: 0,
            finalized: Some(true),
            eligible_ether: None,
            global_participation_rate: None,
            voted_ether: None
        };

        Ok(epoch)
    }
}
