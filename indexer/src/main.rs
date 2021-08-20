use std::env;
use std::time::Instant;

use ::indexer::{epoch_retriever::EpochRetriever, indexer::Indexer};
use ::types::{Epoch, MainnetEthSpec};
use db::{Connection, PgConnection};
use dotenv::dotenv;
use simple_logger::SimpleLogger;

pub mod epoch_retriever;
pub mod errors;
pub mod indexer;
pub mod types;

#[tokio::main]
async fn main() {
    dotenv().ok();
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let endpoint_url = env::var("ENDPOINT_URL").unwrap();

    let db_connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    let epoch_retriever = EpochRetriever::new(endpoint_url);
    let mut epochs = Vec::new();
    let indexer = Indexer::new(db_connection);
    let start = Instant::now();

    for n in 40000..40001 {
        log::info!("Indexing epoch {}", n);

        match epoch_retriever
            .get_consolidated_epoch::<MainnetEthSpec>(Epoch::new(n))
            .await
        {
            Ok(epoch) => epochs.push(epoch),
            Err(err) => log::warn!("Error while indexing epoch {}: {:?}", n, err),
        }
    }

    let duration = start.elapsed();
    log::info!("Avg epoch indexing duration: {:?}", duration.div_f32(10.));

    let res = indexer.index(epochs).await;

    if let Err(err) = res {
        log::error!("Error while persisting in DB: {:?}", err);
    }
}
