use std::fs;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use indexer::node_to_files_indexer::Indexer;
use indexer::retriever::Retriever;

use crate::cli::Cli;

pub async fn process(cli: Cli, running: Arc<AtomicBool>) -> () {
    if cli.reset {
        fs::remove_dir_all("../web/public/data").unwrap();
    }

    fs::create_dir_all("../web/public/data/epochs/e/").unwrap();
    fs::create_dir_all("../web/public/data/epochs/s/attestations_count/").unwrap();
    fs::create_dir_all("../web/public/data/epochs/s/deposits_count/").unwrap();
    fs::create_dir_all("../web/public/data/blocks").unwrap();
    fs::create_dir_all("../web/public/data/blocks/e/").unwrap();
    fs::create_dir_all("../web/public/data/blocks/c/").unwrap();
    fs::create_dir_all("../web/public/data/validators").unwrap();

    let mut retriever = Retriever::new(cli.endpoint_url);
    let mut n = 0;

    while running.load(Ordering::SeqCst) {
        match retriever.retrieve_epoch(n).await {
            Ok(_) => {
                n = n + 1;
            }
            Err(err) => {
                running.store(false, Ordering::SeqCst);
                log::error!("Error while retrieving epoch {}: {:?}", n, err);
            }
        }
    }

    match retriever.retrieve_validators().await {
        Ok(_) => (),
        Err(err) => {
            log::error!("Error while retrieving validators: {:?}", err);
        }
    }

    let indexer = Indexer::from(retriever);

    indexer.index("../web/public/data").unwrap();

    /*
        if let Err(err) = indexer.index_validators().await {
        log::warn!("Error while indexing validators: {:?}", err);
    }
    */
}
