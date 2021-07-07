use diesel::{Connection, PgConnection};

use crate::errors::DbError;

pub struct DbConnection {
    connection: PgConnection,
}

impl DbConnection {
    pub fn new(database_url: &str) -> Result<Self, DbError> {
        let db_connection = DbConnection {
            connection: PgConnection::establish(database_url)?
        };

        Ok(db_connection)
    }
}

impl From<DbConnection> for PgConnection {
    fn from(db_connection: DbConnection) -> Self {
        db_connection.connection
    }
}