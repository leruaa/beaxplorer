use std::sync::Arc;

use db::{ConnectionManager, PgConnection, Pool};
use eth2::types::StateId;
use types::{Epoch, MainnetEthSpec};

use crate::{
    beacon_node_client::BeaconNodeClient,
    persistable::Persistable,
    types::{consolidated_epoch::ConsolidatedEpoch, consolidated_validator::ConsolidatedValidator},
};

pub struct Indexer {
    beacon_client: BeaconNodeClient,
}

impl Indexer {
    pub fn new(endpoint_url: String) -> Self {
        Indexer {
            beacon_client: BeaconNodeClient::new(endpoint_url),
        }
    }

    pub async fn index_epoch(
        &self,
        pool: &Arc<Pool<ConnectionManager<PgConnection>>>,
        number: u64,
    ) {
        log::info!("Indexing epoch {}", number);

        let db_connection = pool.get().expect("Error when getting connection");

        match ConsolidatedEpoch::<MainnetEthSpec>::new(
            Epoch::new(number),
            self.beacon_client.clone(),
        )
        .await
        {
            Ok(epoch) => {
                if let Err(err) = epoch.persist(&db_connection) {
                    log::warn!("Error while persisting epoch {}: {:?}", number, err);
                }
            }
            Err(err) => log::warn!("Error while building epoch {}: {:?}", number, err),
        }
    }

    pub async fn index_validators(&self, pool: &Arc<Pool<ConnectionManager<PgConnection>>>) {
        log::info!("Indexing validators");

        let db_connection = pool.get().expect("Error when getting connection");

        match ConsolidatedValidator::from_state(StateId::Head, self.beacon_client.clone()).await {
            Ok(validators) => {
                if let Err(err) = validators.persist(&db_connection) {
                    log::warn!("Error while persisting validators: {:?}", err);
                }
            }
            Err(err) => log::warn!("Error while building validators: {:?}", err),
        }
    }
}
