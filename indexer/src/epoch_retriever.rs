use eth2::{BeaconNodeHttpClient, types::{BlockId, StateId}};
use sensitive_url::SensitiveUrl;
use types::{Epoch, EthSpec, Signature};
use std::env;

use crate::{errors::IndexerError, types::{consolidated_block::{BlockStatus, ConsolidatedBlock}, consolidated_epoch::ConsolidatedEpoch}};

pub struct EpochRetriever {
    client: BeaconNodeHttpClient,
}

impl EpochRetriever {
    pub fn new() -> Self {
        let endpoint = env::var("ENDPOINT_URL").unwrap();
        let url = SensitiveUrl::parse(&endpoint).unwrap();

        EpochRetriever {
            client: BeaconNodeHttpClient::new(url)
        }
    }

    pub async fn get_consolidated_epoch<E: EthSpec>(&self, epoch: Epoch) -> Result<ConsolidatedEpoch<E>, IndexerError> {
        let mut consolidated_epoch = ConsolidatedEpoch::<E>::new(epoch);
        let mut missed_blocks = Vec::new();

        for slot in epoch.slot_iter(E::slots_per_epoch()) {
            let block_response = self.client.get_beacon_blocks::<E>(BlockId::Slot(slot)).await;
            if let Ok(block_response) = block_response {
                if let Some(block_response) = block_response {
                    let block_root_response = self.client.get_beacon_blocks_root(BlockId::Slot(slot)).await;
                    if let Ok(block_root_response) = block_root_response {
                        if let Some(block_root_response) = block_root_response {
                            consolidated_epoch.blocks.insert(slot, ConsolidatedBlock::new(epoch, slot, Some(block_response.data.message.clone()), block_root_response.data.root, block_response.data.signature, BlockStatus::Proposed, block_response.data.message.proposer_index));
                        }
                    }
                }
                else {
                    missed_blocks.push(slot);
                }
            }
        }

        if missed_blocks.len() > 0
        {
            let proposer_duties = self.client.get_validator_duties_proposer(epoch).await;

            if let Ok(proposer_duties) = proposer_duties {
                for proposer in proposer_duties.data {
                    if missed_blocks.contains(&proposer.slot) {
                        let block_root_response = self.client.get_beacon_blocks_root(BlockId::Slot(proposer.slot)).await;
                        if let Ok(block_root_response) = block_root_response {
                            if let Some(block_root_response) = block_root_response {
                                consolidated_epoch.blocks.insert(proposer.slot, ConsolidatedBlock::new(epoch, proposer.slot, None, block_root_response.data.root,Signature::empty(), BlockStatus::Missed,  proposer.validator_index));
                            }
                        }
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
}