use db::{Connection, PgConnection};
use eth2::types::StateId;
use types::{Epoch, MainnetEthSpec};

use crate::{
    beacon_node_client::BeaconNodeClient,
    persistable::Persistable,
    types::{consolidated_epoch::ConsolidatedEpoch, consolidated_validator::ConsolidatedValidator},
};

pub struct Indexer {
    db_connection: PgConnection,
    client: BeaconNodeClient,
}

impl Indexer {
    pub fn new(database_url: String, endpoint_url: String) -> Self {
        Indexer {
            db_connection: PgConnection::establish(&database_url)
                .expect(&format!("Error connecting to {}", database_url)),
            client: BeaconNodeClient::new(endpoint_url),
        }
    }

    pub async fn index_epoch(&self, number: u64) {
        log::info!("Indexing epoch {}", number);

        match ConsolidatedEpoch::<MainnetEthSpec>::new(Epoch::new(number), self.client.clone())
            .await
        {
            Ok(epoch) => {
                if let Err(err) = epoch.persist(&self.db_connection) {
                    log::warn!("Error while persisting epoch {}: {:?}", number, err);
                }
            }
            Err(err) => log::warn!("Error while building epoch {}: {:?}", number, err),
        }
    }

    pub async fn index_validators(&self) {
        log::info!("Indexing validators");

        match ConsolidatedValidator::from_state(StateId::Head, self.client.clone()).await {
            Ok(validators) => {
                if let Err(err) = validators.persist(&self.db_connection) {
                    log::warn!("Error while persisting validators: {:?}", err);
                }
            }
            Err(err) => log::warn!("Error while building validators: {:?}", err),
        }
    }
}
