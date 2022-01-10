use std::fs;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use indexer::node_to_files_indexer::Indexer;

pub async fn process(endpoint_url: String, running: Arc<AtomicBool>) -> () {
    fs::create_dir_all("../web_static/public/data/epochs/s/attestations_count/").unwrap();
    fs::create_dir_all("../web_static/public/data/blocks").unwrap();

    let mut indexer = Indexer::new(endpoint_url);
    let mut n = 0;

    while running.load(Ordering::SeqCst) {
        match indexer.index_epoch(n).await {
            Ok(_) => {
                n = n + 1;
            }
            Err(err) => {
                running.store(false, Ordering::SeqCst);
                log::error!("Error while indexing epoch {}: {:?}", n, err);
            }
        }
    }

    indexer.finalize();

    /*
        if let Err(err) = indexer.index_validators().await {
        log::warn!("Error while indexing validators: {:?}", err);
    }
    */
}