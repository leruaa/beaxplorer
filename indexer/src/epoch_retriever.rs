use eth2::types::{BlockId, StateId};
use node_client::{NodeClient, config::SLOTS_PER_EPOCHS};
use types::{Epoch, EthSpec};
use std::env;

use crate::types::consolidated_epoch::ConsolidatedEpoch;


pub struct EpochRetriever {
    client: NodeClient,
}

impl EpochRetriever {
    pub fn new() -> Self {
        let endpoint = env::var("ENDPOINT_URL").unwrap();

        EpochRetriever {
            client: NodeClient::new(endpoint)
        }
    }

    pub async fn get_consolidated_epoch<E: EthSpec>(&self, epoch: Epoch) -> Result<ConsolidatedEpoch<E>, ()> {
        let mut consolidated_epoch = ConsolidatedEpoch::<E>::new(epoch);

        for slot in epoch.slot_iter(SLOTS_PER_EPOCHS) {
            let block = self.client.get_block(BlockId::Slot(slot)).await;
            consolidated_epoch.blocks.push(block.unwrap().message);
        }

        let validators_data = self.client.get_validators_from_state(StateId::Slot(epoch.start_slot(SLOTS_PER_EPOCHS))).await;
    
        for validator_data in validators_data.unwrap() {
            consolidated_epoch.validators.push(validator_data.validator);
        }

        Ok(consolidated_epoch)
    }
}