use std::env;
use std::time::Instant;

use ::types::{Epoch, MainnetEthSpec};
use db::{Connection, PgConnection};
use dotenv::dotenv;
use eth2::types::StateId;
use indexer::beacon_node_client::BeaconNodeClient;
use indexer::persistable::Persistable;
use indexer::types::consolidated_epoch::ConsolidatedEpoch;
use indexer::types::consolidated_validator::ConsolidatedValidator;
use simple_logger::SimpleLogger;

pub mod beacon_node_client;
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
    let client = BeaconNodeClient::new(endpoint_url);
    let start = Instant::now();

    for n in 40000..40010 {
        log::info!("Indexing epoch {}", n);

        match ConsolidatedEpoch::<MainnetEthSpec>::new(Epoch::new(n), client.clone()).await {
            Ok(epoch) => {
                if let Err(err) = epoch.persist(&db_connection) {
                    log::warn!("Error while persisting epoch {}: {:?}", n, err);
                }
            }
            Err(err) => log::warn!("Error while building epoch {}: {:?}", n, err),
        }
    }

    log::info!("Indexing validators");

    match ConsolidatedValidator::from_state(StateId::Head, client).await {
        Ok(validators) => {
            if let Err(err) = validators.persist(&db_connection) {
                log::warn!("Error while persisting validators: {:?}", err);
            }
        }
        Err(err) => log::warn!("Error while building validators: {:?}", err),
    }

    let duration = start.elapsed();
    log::info!("Avg epoch indexing duration: {:?}", duration.div_f32(10.));
}
