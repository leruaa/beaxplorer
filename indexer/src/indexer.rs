use db::{PgConnection, RunQueryDsl};
use types::{EthSpec};

use crate::{errors::IndexerError, types::consolidated_epoch::ConsolidatedEpoch};
use db::schema::epochs::dsl::*;
use db::schema::blocks::dsl::*;

pub struct Indexer {
    db_connection: PgConnection,
}

impl Indexer {
    pub fn new(db_connection: PgConnection) -> Self {
        Indexer {
            db_connection,
        }
    }

    pub async fn index<E: EthSpec>(&self, consolidated_epochs: Vec<ConsolidatedEpoch<E>>) -> Result<(), IndexerError> {

        for consolidated_epoch in consolidated_epochs {
            let epoch_model = consolidated_epoch.as_model()?;

            db::insert_into(epochs).values(epoch_model).execute(&self.db_connection)?;

            for consolidated_block in consolidated_epoch.blocks {
                let block_model = consolidated_block.as_model()?;
                db::insert_into(blocks).values(block_model).execute(&self.db_connection)?;
            }
        }

        Ok(())
    }
}