use eth2::types::StateId;
use lighthouse_types::{Epoch, MainnetEthSpec};

use crate::{
    beacon_node_client::BeaconNodeClient,
    errors::IndexerError,
    types::{consolidated_epoch::ConsolidatedEpoch, consolidated_validator::ConsolidatedValidator},
};

pub struct Retriever {
    beacon_client: BeaconNodeClient,
    pub epochs: Vec<ConsolidatedEpoch<MainnetEthSpec>>,
    pub validators: Vec<ConsolidatedValidator>,
}

impl Retriever {
    pub fn new(endpoint_url: String) -> Self {
        Retriever {
            beacon_client: BeaconNodeClient::new(endpoint_url),
            epochs: Vec::new(),
            validators: Vec::new(),
        }
    }

    pub async fn retrieve_epoch(&mut self, number: u64) -> Result<(), IndexerError> {
        log::info!("Retrieving epoch {}", number);

        self.epochs.push(
            ConsolidatedEpoch::<MainnetEthSpec>::new(
                Epoch::new(number),
                self.beacon_client.clone(),
            )
            .await?,
        );

        Ok(())
    }

    pub async fn retrieve_validators(&mut self) -> Result<(), IndexerError> {
        log::info!("Retrieving validators");

        self.validators.extend(
            ConsolidatedValidator::from_state(StateId::Head, self.beacon_client.clone()).await?,
        );

        Ok(())
    }
}
