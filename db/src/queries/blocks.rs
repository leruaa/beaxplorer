use diesel::QueryDsl;

use crate::models::BlockModel;

use super::super::schema::blocks;
pub use diesel::prelude::*;

pub fn by_slot<'a>(s: i64, connection: &PgConnection) -> QueryResult<BlockModel> {
    blocks::table
        .limit(1)
        .filter(blocks::dsl::slot.eq_all(s))
        .first(connection)
}

pub fn get_latests<'a>(limit: i64, connection: &PgConnection) -> QueryResult<Vec<BlockModel>> {
    blocks::table
        .limit(limit)
        .order(blocks::slot)
        .load(connection)
}
