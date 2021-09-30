use diesel::{dsl::Find, QueryDsl};

use super::super::schema::validators;

pub fn by_number<'a>(number: i32) -> Find<validators::table, i32> {
    validators::table.find(number)
}

pub fn get_latests<'a>(limit: i64) -> validators::BoxedQuery<'a, diesel::pg::Pg> {
    validators::table
        .limit(limit)
        .order(validators::validator_index)
        .into_boxed()
}
