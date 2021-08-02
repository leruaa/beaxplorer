use diesel::QueryDsl;

use super::super::schema::epochs;

pub fn get_latests<'a>(limit: i64) -> epochs::BoxedQuery<'a, diesel::pg::Pg> {
    epochs::table.limit(limit).order(epochs::epoch).into_boxed()
}
