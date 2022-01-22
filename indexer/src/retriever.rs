use eth2::types::StateId;
use lighthouse_types::{Epoch, MainnetEthSpec};
use types::{block::BlockModel, epoch::EpochModel, validator::ValidatorModel};

use crate::{
    beacon_node_client::BeaconNodeClient,
    errors::IndexerError,
    types::{consolidated_epoch::ConsolidatedEpoch, consolidated_validator::ConsolidatedValidator},
};

pub struct Retriever {
    beacon_client: BeaconNodeClient,
    pub epochs: Vec<EpochModel>,
    pub blocks: Vec<BlockModel>,
    pub validators: Vec<ValidatorModel>,
}

impl Retriever {
    pub fn new(endpoint_url: String) -> Self {
        Retriever {
            beacon_client: BeaconNodeClient::new(endpoint_url),
            epochs: Vec::new(),
            blocks: Vec::new(),
            validators: Vec::new(),
        }
    }

    pub async fn retrieve_epoch(&mut self, number: u64) -> Result<(), IndexerError> {
        log::info!("Retrieving epoch {}", number);

        let epoch = ConsolidatedEpoch::<MainnetEthSpec>::new(
            Epoch::new(number),
            self.beacon_client.clone(),
        )
        .await?;

        self.blocks.extend(
            epoch
                .blocks
                .clone()
                .into_iter()
                .map(|x| BlockModel::from(x)),
        );
        self.epochs.push(EpochModel::from(epoch));

        Ok(())
    }

    pub async fn retrieve_validators(&mut self) -> Result<(), IndexerError> {
        log::info!("Retrieving validators");

        self.validators.extend(
            ConsolidatedValidator::from_state(StateId::Head, self.beacon_client.clone())
                .await?
                .into_iter()
                .map(|x| ValidatorModel::from(x))
                .collect::<Vec<ValidatorModel>>(),
        );

        Ok(())
    }
}
