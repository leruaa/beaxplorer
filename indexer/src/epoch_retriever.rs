use eth2::types::BlockId;
use node_client::NodeClient;
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
        
        for slot in epoch.slot_iter(node_client::config::SLOTS_PER_EPOCHS) {
            let block = self.client.get_block(BlockId::Slot(slot)).await;
            consolidated_epoch.blocks.push(block.unwrap().message);
        }
    
        Ok(consolidated_epoch)
    }
}