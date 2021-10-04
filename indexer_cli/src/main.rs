use std::env;
use std::time::Instant;

use dotenv::dotenv;
use indexer::indexer::Indexer;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    dotenv().ok();
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let endpoint_url = env::var("ENDPOINT_URL").unwrap();
    let start = Instant::now();

    let indexer = Indexer::new(database_url, endpoint_url);

    for n in 40000..40010 {
        indexer.index_epoch(n).await;
    }

    indexer.index_validators().await;

    let duration = start.elapsed();
    log::info!("Avg epoch indexing duration: {:?}", duration.div_f32(10.));
}
