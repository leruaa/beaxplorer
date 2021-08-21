use std::env;
use std::time::Instant;

use ::indexer::epoch_retriever::EpochRetriever;
use ::types::{Epoch, MainnetEthSpec};
use db::{Connection, PgConnection};
use dotenv::dotenv;
use indexer::persistable::Persistable;
use simple_logger::SimpleLogger;

pub mod beacon_node_client;
pub mod epoch_retriever;
pub mod errors;
pub mod persistable;
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
    let start = Instant::now();

    for n in 40000..40010 {
        log::info!("Indexing epoch {}", n);

        match epoch_retriever
            .get_consolidated_epoch::<MainnetEthSpec>(Epoch::new(n))
            .await
        {
            Ok(epoch) => {
                if let Err(err) = epoch.persist(&db_connection) {
                    log::warn!("Error while persisting epoch {}: {:?}", n, err);
                }
            }
            Err(err) => log::warn!("Error while indexing epoch {}: {:?}", n, err),
        }
    }

    let duration = start.elapsed();
    log::info!("Avg epoch indexing duration: {:?}", duration.div_f32(10.));
}
