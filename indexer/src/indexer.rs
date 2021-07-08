use db::{PgConnection, RunQueryDsl};
use types::{Epoch, MainnetEthSpec};

use crate::{epoch_retriever::EpochRetriever, errors::IndexerError};
use db::schema::epochs::dsl::*;

pub struct Indexer {
    db_connection: PgConnection,
    epoch_retriever: EpochRetriever,

}

impl Indexer {
    pub fn new(db_connection: PgConnection) -> Self {
        Indexer {
            db_connection,
            epoch_retriever: EpochRetriever::new(),
        }
    }

    pub async fn index(&self) -> Result<(), IndexerError> {
        let consolidated_epoch = self.epoch_retriever.get_consolidated_epoch::<MainnetEthSpec>(Epoch::new(40000)).await?;
        let record = consolidated_epoch.as_model()?;

        db::insert_into(epochs).values(record).execute(&self.db_connection)?;

        Ok(())
    }
}