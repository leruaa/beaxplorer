use std::{sync::Arc, time::Instant};

use eth2::types::ProposerData;
use futures::future::try_join_all;
use tokio::sync::RwLock;
use types::{Epoch, EthSpec, Hash256, Signature, Slot};

use crate::{
    beacon_node_client::BeaconNodeClient,
    errors::IndexerError,
    types::{
        consolidated_block::{BlockStatus, ConsolidatedBlock},
        consolidated_epoch::ConsolidatedEpoch,
    },
};

pub struct EpochRetriever {
    client: BeaconNodeClient,
}

impl EpochRetriever {
    pub fn new(endpoint_url: String) -> Self {
        EpochRetriever {
            client: BeaconNodeClient::new(endpoint_url),
        }
    }

    pub async fn get_consolidated_epoch<E: EthSpec>(
        &self,
        epoch: Epoch,
    ) -> Result<ConsolidatedEpoch<E>, IndexerError> {
        let mut build_consolidated_block_futures = Vec::new();
        let proposer_duties_lock = Arc::new(RwLock::new(Option::<Vec<ProposerData>>::None));

        let get_validator_balances_handle = tokio::spawn(
            self.client
                .get_validators_balances(epoch.start_slot(E::slots_per_epoch())),
        );

        let get_validator_inclusion_handle =
            tokio::spawn(self.client.get_validator_inclusion(epoch));

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
            validator_balances: get_validator_balances_handle.await??,
            validator_inclusion: get_validator_inclusion_handle.await??,
        })
    }

    async fn build_consolidated_block<E: EthSpec>(
        &self,
        epoch: Epoch,
        slot: Slot,
        proposer_duties_lock: Arc<RwLock<Option<Vec<ProposerData>>>>,
    ) -> Result<ConsolidatedBlock<E>, IndexerError> {
        let start = Instant::now();
        let block_response = self.client.get_block::<E>(slot).await?;
        let duration = start.elapsed();
        log::trace!("get_block duration: {:?}", duration);

        if let Some(block_response) = block_response {
            let start = Instant::now();
            let block_root = self
                .client
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
                proposer_duties_writable
                    .replace(self.client.get_validator_duties_proposer(epoch).await?);
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
}
