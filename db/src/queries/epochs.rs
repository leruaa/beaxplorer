use diesel::{dsl::max, QueryDsl};

use crate::{models::EpochModel, schema::epochs::dsl::*};
use diesel::prelude::*;

pub fn by_number<'a>(e: i64, connection: &PgConnection) -> QueryResult<EpochModel> {
    epochs.find(e).first(connection)
}

pub fn get_latests<'a>(limit: i64, connection: &PgConnection) -> QueryResult<Vec<EpochModel>> {
    epochs.limit(limit).order(epoch).load(connection)
}

pub fn get_latest_finalized_epoch<'a>(connection: &PgConnection) -> QueryResult<Option<i64>> {
    epochs
        .select(max(epoch))
        .filter(finalized.eq_all(true))
        .get_result(connection)
}
