use std::sync::Arc;

use db::{ConnectionManager, PgConnection, Pool};
use indexer::db_to_files_indexer::Indexer;

pub fn process(db_pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> () {
    let indexer = Indexer {};

    indexer.persist_epochs(&db_pool);
}
