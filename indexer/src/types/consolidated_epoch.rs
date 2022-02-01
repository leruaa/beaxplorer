use std::ops::Div;
use std::sync::Arc;

use eth2::lighthouse::GlobalValidatorInclusionData;
use eth2::types::{CommitteeData, ProposerData, StateId, ValidatorBalanceData};
use futures::future::try_join_all;
use lighthouse_types::{Epoch, EthSpec};
use shared::utils::clock::Clock;
use tokio::sync::RwLock;
use types::epoch::{EpochExtendedModel, EpochExtendedModelWithId, EpochModel, EpochModelWithId};

use crate::beacon_node_client::BeaconNodeClient;
use crate::errors::IndexerError;

use super::consolidated_block::ConsolidatedBlock;

#[derive(Debug)]
pub struct ConsolidatedEpoch<E: EthSpec> {
    pub epoch: Epoch,
    pub blocks: Vec<ConsolidatedBlock<E>>,
    pub validator_balances: Vec<ValidatorBalanceData>,
    pub validator_inclusion: GlobalValidatorInclusionData,
    pub committees: Arc<RwLock<Vec<CommitteeData>>>,
}

impl<E: EthSpec> ConsolidatedEpoch<E> {
    pub async fn new(epoch: Epoch, client: BeaconNodeClient) -> Result<Self, IndexerError> {
        let mut build_consolidated_block_futures = Vec::new();
        let proposer_duties_lock = Arc::new(RwLock::new(Option::<Vec<ProposerData>>::None));

        let get_validator_balances_handle = tokio::spawn(
            client.get_validators_balances(StateId::Slot(epoch.start_slot(E::slots_per_epoch()))),
        );

        let get_validator_inclusion_handle = tokio::spawn(client.get_validator_inclusion(epoch));

        let get_committees_handle = tokio::spawn(client.get_committees(epoch));

        let committees_lock = Arc::new(RwLock::new(get_committees_handle.await??));

        for slot in epoch.slot_iter(E::slots_per_epoch()) {
            build_consolidated_block_futures.push(ConsolidatedBlock::new(
                epoch,
                slot,
                proposer_duties_lock.clone(),
                committees_lock.clone(),
                client.clone(),
            ));
        }

        Ok(ConsolidatedEpoch::<E> {
            epoch,
            blocks: try_join_all(build_consolidated_block_futures).await?,
            validator_balances: get_validator_balances_handle.await??,
            validator_inclusion: get_validator_inclusion_handle.await??,
            committees: committees_lock,
        })
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
            eligible_ether: eligible_ether,
            voted_ether: voted_ether,
        };

        (value.epoch.as_u64(), model)
    }
}

impl<E: EthSpec> From<&ConsolidatedEpoch<E>> for EpochExtendedModelWithId {
    fn from(value: &ConsolidatedEpoch<E>) -> Self {
        let model = EpochExtendedModel {
            voluntary_exits_count: value.get_voluntary_exits_count(),
            validators_count: value.validator_balances.len(),
            average_validator_balance: value
                .get_total_validator_balance()
                .div(value.validator_balances.len() as u64),
            total_validator_balance: value.get_total_validator_balance(),
        };

        (value.epoch.as_u64(), model)
    }
}
