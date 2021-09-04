use diesel::QueryDsl;

use super::super::schema::validators;

pub fn get_latests<'a>(limit: i64) -> validators::BoxedQuery<'a, diesel::pg::Pg> {
    validators::table
        .limit(limit)
        .order(validators::validator_index)
        .into_boxed()
}
