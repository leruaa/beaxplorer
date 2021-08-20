use std::{sync::Arc, time::Instant};

use eth2::{
    lighthouse::GlobalValidatorInclusionData,
    types::{BlockId, GenericResponse, ProposerData, RootData, StateId, ValidatorData},
    BeaconNodeHttpClient,
};
use futures::{future::try_join_all, Future};
use sensitive_url::SensitiveUrl;
use tokio::sync::RwLock;
use types::{Epoch, EthSpec, Hash256, Signature, SignedBeaconBlock, Slot};

use crate::{
    errors::IndexerError,
    types::{
        consolidated_block::{BlockStatus, ConsolidatedBlock},
        consolidated_epoch::ConsolidatedEpoch,
        consolidated_validator::ConsolidatedValidator,
    },
};

pub struct EpochRetriever {
    client: BeaconNodeHttpClient,
}

impl EpochRetriever {
    pub fn new(endpoint_url: String) -> Self {
        let url = SensitiveUrl::parse(&endpoint_url).unwrap();

        EpochRetriever {
            client: BeaconNodeHttpClient::new(url),
        }
    }

    pub async fn get_consolidated_epoch<E: EthSpec>(
        &self,
        epoch: Epoch,
    ) -> Result<ConsolidatedEpoch<E>, IndexerError> {
        let mut build_consolidated_block_futures = Vec::new();
        let proposer_duties_lock = Arc::new(RwLock::new(Option::<Vec<ProposerData>>::None));

        let get_validators_handle =
            tokio::spawn(self.get_validators(epoch.start_slot(E::slots_per_epoch())));

        let get_validator_inclusion_handle = tokio::spawn(self.get_validator_inclusion(epoch));

        for slot in epoch.slot_iter(E::slots_per_epoch()) {
            build_consolidated_block_futures.push(self.clone().build_consolidated_block::<E>(
                epoch,
                slot,
                proposer_duties_lock.clone(),
            ));
        }

        Ok(ConsolidatedEpoch::<E> {
            epoch,
            blocks: try_join_all(build_consolidated_block_futures).await?,
            validators: get_validators_handle
                .await??
                .into_iter()
                .map(|v| ConsolidatedValidator(v))
                .collect(),
            validator_inclusion: get_validator_inclusion_handle.await??,
        })
    }

    async fn get_block<E: EthSpec>(
        &self,
        slot: Slot,
    ) -> Result<Option<GenericResponse<SignedBeaconBlock<E>>>, IndexerError> {
        self.client
            .get_beacon_blocks::<E>(BlockId::Slot(slot))
            .await
            .map_err(|inner_error| IndexerError::NodeError { inner_error })
    }

    async fn get_block_root(&self, slot: Slot) -> Result<GenericResponse<RootData>, IndexerError> {
        self.client
            .get_beacon_blocks_root(BlockId::Slot(slot))
            .await
            .map_err(|inner_error| IndexerError::NodeError { inner_error })?
            .ok_or(IndexerError::ElementNotFound(slot))
    }

    async fn build_consolidated_block<E: EthSpec>(
        &self,
        epoch: Epoch,
        slot: Slot,
        proposer_duties_lock: Arc<RwLock<Option<Vec<ProposerData>>>>,
    ) -> Result<ConsolidatedBlock<E>, IndexerError> {
        let start = Instant::now();
        let block_response = self.get_block::<E>(slot).await?;
        let duration = start.elapsed();
        log::trace!("get_block duration: {:?}", duration);

        if let Some(block_response) = block_response {
            let start = Instant::now();
            let block_root = self
                .get_block_root(block_response.data.message.slot)
                .await?;
            let duration = start.elapsed();
            log::trace!("get_block_root duration: {:?}", duration);
            let consolidated_block = ConsolidatedBlock::new(
                epoch,
                block_response.data.message.slot,
                Some(block_response.data.message.clone()),
                block_root.data.root,
                block_response.data.signature,
                BlockStatus::Proposed,
                block_response.data.message.proposer_index,
            );

            return Ok(consolidated_block);
        } else {
            let mut proposer_duties = proposer_duties_lock.read().await.clone();

            if proposer_duties.is_none() {
                let mut proposer_duties_writable = proposer_duties_lock.write().await;
                proposer_duties_writable.replace(self.get_validator_duties_proposer(epoch).await?);
                proposer_duties = proposer_duties_writable.clone();
            }

            if let Some(proposer_duties) = proposer_duties {
                for proposer in proposer_duties {
                    if proposer.slot == slot {
                        let consolidated_block = ConsolidatedBlock::new(
                            epoch,
                            proposer.slot,
                            None,
                            Hash256::zero(),
                            Signature::empty(),
                            BlockStatus::Missed,
                            proposer.validator_index,
                        );

                        return Ok(consolidated_block);
                    }
                }
            }
        }

        Err(IndexerError::ElementNotFound(slot))
    }

    fn get_validators(
        &self,
        slot: Slot,
    ) -> impl Future<Output = Result<Vec<ValidatorData>, IndexerError>> {
        let client = self.client.clone();

        async move {
            client
                .get_beacon_states_validators(StateId::Slot(slot), None, None)
                .await
                .transpose()
                .ok_or(IndexerError::ElementNotFound(slot))?
                .map(|response| response.data)
                .map_err(|inner_error| IndexerError::NodeError { inner_error })
        }
    }

    async fn get_validator_duties_proposer(
        &self,
        epoch: Epoch,
    ) -> Result<Vec<ProposerData>, IndexerError> {
        log::trace!("Getting duties proposer for epoch {}", epoch);
        self.client
            .get_validator_duties_proposer(epoch)
            .await
            .map(|response| response.data)
            .map_err(|inner_error| IndexerError::NodeError { inner_error })
    }

    fn get_validator_inclusion(
        &self,
        epoch: Epoch,
    ) -> impl Future<Output = Result<GlobalValidatorInclusionData, IndexerError>> {
        let client = self.client.clone();

        async move {
            client
                .get_lighthouse_validator_inclusion_global(epoch)
                .await
                .map(|response| response.data)
                .map_err(|inner_error| IndexerError::NodeError { inner_error })
        }
    }
}
