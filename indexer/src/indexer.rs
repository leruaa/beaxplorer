use std::sync::Arc;

use db::{ConnectionManager, PgConnection, Pool};
use eth2::types::StateId;
use types::{Epoch, MainnetEthSpec};

use crate::{
    beacon_node_client::BeaconNodeClient,
    errors::IndexerError,
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

    pub async fn get_latest_indexed_epoch(
        &self,
        pool: &Arc<Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Option<u64>, IndexerError> {
        let db_connection = pool.get().expect("Error when getting connection");

        let latest_finalized_epoch =
            db::queries::epochs::get_latest_finalized_epoch(&db_connection)?;

        Ok(latest_finalized_epoch.map(|n| n as u64))
    }

    pub async fn index_epoch(
        &self,
        pool: &Arc<Pool<ConnectionManager<PgConnection>>>,
        number: u64,
    ) -> Result<(), IndexerError> {
        log::info!("Indexing epoch {}", number);

        let db_connection = pool.get().expect("Error when getting connection");

        let epoch = ConsolidatedEpoch::<MainnetEthSpec>::new(
            Epoch::new(number),
            self.beacon_client.clone(),
        )
        .await?;

        epoch.persist(&db_connection)?;

        Ok(())
    }

    pub async fn index_validators(
        &self,
        pool: &Arc<Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<(), IndexerError> {
        log::info!("Indexing validators");

        let db_connection = pool.get().expect("Error when getting connection");

        let validators =
            ConsolidatedValidator::from_state(StateId::Head, self.beacon_client.clone()).await?;

        validators.persist(&db_connection)?;

        Ok(())
    }
}
