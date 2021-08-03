use diesel::{dsl::Find, QueryDsl};

use super::super::schema::epochs;

pub fn by_number<'a>(epoch: i64) -> Find<epochs::table, i64> {
    epochs::table.find(epoch)
}

pub fn get_latests<'a>(limit: i64) -> epochs::BoxedQuery<'a, diesel::pg::Pg> {
    epochs::table.limit(limit).order(epochs::epoch).into_boxed()
}
