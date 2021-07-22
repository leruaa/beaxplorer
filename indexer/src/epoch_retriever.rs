use eth2::{BeaconNodeHttpClient, types::{BlockId, GenericResponse, RootData, StateId}};
use sensitive_url::SensitiveUrl;
use types::{Epoch, EthSpec, Hash256, Signature, SignedBeaconBlock, Slot};

use crate::{errors::IndexerError, types::{consolidated_block::{BlockStatus, ConsolidatedBlock}, consolidated_epoch::ConsolidatedEpoch}};

pub struct EpochRetriever {
    client: BeaconNodeHttpClient,
}

impl EpochRetriever {
    pub fn new(endpoint_url: String) -> Self {
        let url = SensitiveUrl::parse(&endpoint_url).unwrap();

        EpochRetriever {
            client: BeaconNodeHttpClient::new(url)
        }
    }

    pub async fn get_consolidated_epoch<E: EthSpec>(&self, epoch: Epoch) -> Result<ConsolidatedEpoch<E>, IndexerError> {
        let mut consolidated_epoch = ConsolidatedEpoch::<E>::new(epoch);
        let mut missed_blocks = Vec::new();

        for slot in epoch.slot_iter(E::slots_per_epoch()) {
            let block_response = self.get_block::<E>(slot).await?;
            if let Some(block_response) = block_response {
                let block_root = self.get_block_root(slot).await?;
                consolidated_epoch.blocks.push(ConsolidatedBlock::new(epoch, slot, Some(block_response.data.message.clone()), block_root.data.root, block_response.data.signature, BlockStatus::Proposed, block_response.data.message.proposer_index));
            }
            else {
                missed_blocks.push(slot);
            }
        }

        if missed_blocks.len() > 0
        {
            let proposer_duties = self.client.get_validator_duties_proposer(epoch).await;

            if let Ok(proposer_duties) = proposer_duties {
                for proposer in proposer_duties.data {
                    if missed_blocks.contains(&proposer.slot) {
                        consolidated_epoch.blocks.push(ConsolidatedBlock::new(epoch, proposer.slot, None, Hash256::zero(),Signature::empty(), BlockStatus::Missed,  proposer.validator_index));
                    }
                }
            }
        }

        let response = self.client.get_beacon_states_validators(StateId::Slot(epoch.start_slot(E::slots_per_epoch())), None, None).await;
    
        if let Ok(response) = response {
            if let Some(response) = response {
                for validator_data in response.data {
                    consolidated_epoch.validators.push(validator_data.validator);
                }
            }
        }

        Ok(consolidated_epoch)
    }

    async fn get_block<E: EthSpec>(&self, slot: Slot) -> Result<Option<GenericResponse<SignedBeaconBlock<E>>>, IndexerError> {
        self.client.get_beacon_blocks::<E>(BlockId::Slot(slot))
            .await
            .map_err(|inner_error| IndexerError::NodeError { inner_error })
    }

    async fn get_block_root(&self, slot: Slot) -> Result<GenericResponse<RootData>, IndexerError> {
        self.client.get_beacon_blocks_root(BlockId::Slot(slot))
            .await
            .map_err(|inner_error| IndexerError::NodeError { inner_error })?
            .ok_or(IndexerError::ElementNotFound(slot))
    }
}