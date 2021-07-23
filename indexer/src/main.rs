use std::env;

use db::{Connection, PgConnection};
use dotenv::dotenv;
use ::indexer::{epoch_retriever::EpochRetriever, indexer::Indexer};
use log::info;
use simple_logger::SimpleLogger;
use ::types::{Epoch, MainnetEthSpec};

pub mod indexer;
pub mod epoch_retriever;
pub mod types;
pub mod errors;

#[tokio::main]
async fn main() {
    dotenv().ok();
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let endpoint_url = env::var("ENDPOINT_URL").unwrap();

    let db_connection = PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
    let epoch_retriever = EpochRetriever::new(endpoint_url);
    let mut epochs = Vec::new();
    let indexer = Indexer::new(db_connection);

    for n in 40000..40010 {
        log::info!("Indexing epoch {}", n);
        if let Ok(epoch) = epoch_retriever.get_consolidated_epoch::<MainnetEthSpec>(Epoch::new(n)).await {
            epochs.push(epoch);
        }
    }

    indexer.index(epochs).await;
}