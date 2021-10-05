use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

use db::{ConnectionManager, PgConnection, Pool};
use dotenv::dotenv;
use indexer::indexer::Indexer;
use simple_logger::SimpleLogger;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    dotenv().ok();
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let endpoint_url = env::var("ENDPOINT_URL").unwrap();
    let db_pool = Arc::new(
        Pool::new(ConnectionManager::<PgConnection>::new(&database_url))
            .expect(&format!("Error connecting to {}", database_url)),
    );
    let start = Instant::now();

    let running = Arc::new(AtomicBool::new(true));

    let (sender, receiver) = oneshot::channel::<()>();
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    tokio::spawn(async move {
        let indexer = Indexer::new(endpoint_url);
        let mut n = indexer
            .get_latest_indexed_epoch(&db_pool.clone())
            .await
            .unwrap()
            .map(|n| n + 1)
            .unwrap_or_else(|| 0);

        while running.load(Ordering::SeqCst) {
            indexer.index_epoch(&db_pool.clone(), n).await;
            n = n + 1;
        }

        indexer.index_validators(&db_pool.clone()).await;

        sender.send(()).unwrap();
    });

    receiver.await.unwrap();

    let duration = start.elapsed();
    log::info!("Avg epoch indexing duration: {:?}", duration.div_f32(10.));
}
