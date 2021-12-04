use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use db::{ConnectionManager, PgConnection, Pool};
use indexer::node_to_db_indexer::Indexer;

pub async fn process(
    endpoint_url: String,
    db_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    running: Arc<AtomicBool>,
) -> () {
    let indexer = Indexer::new(endpoint_url);
    let mut n = indexer
        .get_latest_indexed_epoch(&db_pool.clone())
        .await
        .unwrap()
        .map(|n| n + 1)
        .unwrap_or_else(|| 0);

    while running.load(Ordering::SeqCst) {
        match indexer.index_epoch(&db_pool.clone(), n).await {
            Ok(_) => {
                n = n + 1;
            }
            Err(err) => {
                running.store(false, Ordering::SeqCst);
                log::error!("Error while indexing epoch {}: {:?}", n, err);
            }
        }
    }

    if let Err(err) = indexer.index_validators(&db_pool.clone()).await {
        log::warn!("Error while indexing validators: {:?}", err);
    }
}
