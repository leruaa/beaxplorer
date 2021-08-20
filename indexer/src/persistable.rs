use db::PgConnection;

use crate::{ errors::IndexerError};

pub trait Persistable {
    fn persist(&self, db_connection: &PgConnection) -> Result<(), IndexerError>;
}
