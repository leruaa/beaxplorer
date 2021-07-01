use eth2::{BeaconNodeHttpClient, types::{BlockId, StateId}};
use node_client::config::SLOTS_PER_EPOCHS;
use sensitive_url::SensitiveUrl;
use types::{Epoch, EthSpec};
use std::env;

use crate::types::consolidated_epoch::ConsolidatedEpoch;


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

    pub async fn get_consolidated_epoch<E: EthSpec>(&self, epoch: Epoch) -> Result<ConsolidatedEpoch<E>, ()> {
        let mut consolidated_epoch = ConsolidatedEpoch::<E>::new(epoch);

        for slot in epoch.slot_iter(SLOTS_PER_EPOCHS) {
            let response = self.client.get_beacon_blocks::<E>(BlockId::Slot(slot)).await;
            if let Ok(response) = response {
                if let Some(response) = response {
                    consolidated_epoch.blocks.push(response.data.message);
                }
            }
        }

        let response = self.client.get_beacon_states_validators(StateId::Slot(epoch.start_slot(SLOTS_PER_EPOCHS)), None, None).await;
    
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