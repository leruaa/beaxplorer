use std::ops::Div;
use std::sync::Arc;

use db::models::EpochModel;
use eth2::lighthouse::GlobalValidatorInclusionData;
use eth2::types::{ProposerData, StateId, ValidatorBalanceData};
use futures::future::try_join_all;
use lighthouse_types::{Epoch, EthSpec};
use shared::utils::clock::Clock;
use shared::utils::convert::IntoClampedI64;
use tokio::sync::RwLock;
use types::views::EpochView;

use crate::beacon_node_client::BeaconNodeClient;
use crate::errors::IndexerError;

use super::consolidated_block::ConsolidatedBlock;

#[derive(Debug)]
pub struct ConsolidatedEpoch<E: EthSpec> {
    pub epoch: Epoch,
    pub blocks: Vec<ConsolidatedBlock<E>>,
    pub validator_balances: Vec<ValidatorBalanceData>,
    pub validator_inclusion: GlobalValidatorInclusionData,
}

impl<E: EthSpec> ConsolidatedEpoch<E> {
    pub async fn new(epoch: Epoch, client: BeaconNodeClient) -> Result<Self, IndexerError> {
        let mut build_consolidated_block_futures = Vec::new();
        let proposer_duties_lock = Arc::new(RwLock::new(Option::<Vec<ProposerData>>::None));

        let get_validator_balances_handle = tokio::spawn(
            client.get_validators_balances(StateId::Slot(epoch.start_slot(E::slots_per_epoch()))),
        );

        let get_validator_inclusion_handle = tokio::spawn(client.get_validator_inclusion(epoch));

        for slot in epoch.slot_iter(E::slots_per_epoch()) {
            build_consolidated_block_futures.push(ConsolidatedBlock::new(
                epoch,
                slot,
                proposer_duties_lock.clone(),
                client.clone(),
            ));
        }

        Ok(ConsolidatedEpoch::<E> {
            epoch,
            blocks: try_join_all(build_consolidated_block_futures).await?,
            validator_balances: get_validator_balances_handle.await??,
            validator_inclusion: get_validator_inclusion_handle.await??,
        })
    }

    pub fn as_model(&self) -> Result<EpochModel, IndexerError> {
        let epoch = self.epoch.as_u64().into_i64();
        let total_validator_balance: i64 = self.get_total_validator_balance().into_i64();
        let eligible_ether = self
            .validator_inclusion
            .previous_epoch_active_gwei
            .into_i64();
        let voted_ether = self
            .validator_inclusion
            .previous_epoch_target_attesting_gwei
            .into_i64();
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
            validators_count: self.validator_balances.len() as i32,
            average_validator_balance: total_validator_balance
                .div(self.validator_balances.len() as i64),
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
        self.validator_balances.iter().map(|v| v.balance).sum()
    }
}

impl<E: EthSpec> From<ConsolidatedEpoch<E>> for EpochView {
    fn from(value: ConsolidatedEpoch<E>) -> Self {
        let start_slot = value.epoch.start_slot(E::slots_per_epoch());
        let spec = E::default_spec();
        let clock = Clock::new(spec);

        let eligible_ether = value.validator_inclusion.previous_epoch_active_gwei;
        let voted_ether = value
            .validator_inclusion
            .previous_epoch_target_attesting_gwei;
        let global_participation_rate = (value
            .validator_inclusion
            .previous_epoch_target_attesting_gwei as f64)
            .div(value.validator_inclusion.previous_epoch_active_gwei as f64);

        EpochView {
            epoch: value.epoch.as_u64(),
            timestamp: clock.timestamp(start_slot).unwrap_or(0),
            blocks_count: value.blocks.len(),
            proposer_slashings_count: value.get_proposer_slashings_count(),
            attester_slashings_count: value.get_attester_slashings_count(),
            attestations_count: value.get_attestations_count(),
            deposits_count: value.get_deposits_count(),
            voluntary_exits_count: value.get_voluntary_exits_count(),
            validators_count: value.validator_balances.len(),
            average_validator_balance: value
                .get_total_validator_balance()
                .div(value.validator_balances.len() as u64),
            total_validator_balance: value.get_total_validator_balance(),
            finalized: global_participation_rate >= 2f64 / 3f64,
            eligible_ether: Some(eligible_ether),
            global_participation_rate: Some(global_participation_rate),
            voted_ether: Some(voted_ether),
        }
    }
}
