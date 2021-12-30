use std::{fs, sync::Arc};

use db::{ConnectionManager, PgConnection, Pool};
use indexer::db_to_files_indexer::Indexer;

pub fn process(db_pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> () {
    let indexer = Indexer {};

    fs::create_dir_all("../web_static/public/data/epochs/s/attestations_count/").unwrap();

    indexer.persist_epochs(&db_pool);
}
