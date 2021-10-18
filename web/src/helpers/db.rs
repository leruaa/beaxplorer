use rocket_sync_db_pools::database;

#[database("node")]
pub struct NodeDbConn(db::PgConnection);
