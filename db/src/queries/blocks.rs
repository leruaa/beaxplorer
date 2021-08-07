use diesel::QueryDsl;

use super::super::schema::blocks;
pub use diesel::prelude::*;

pub fn by_slot<'a>(s: i64) -> blocks::BoxedQuery<'a, diesel::pg::Pg> {
    blocks::table
        .filter(blocks::dsl::slot.eq_all(s))
        .into_boxed()
}

pub fn get_latests<'a>(limit: i64) -> blocks::BoxedQuery<'a, diesel::pg::Pg> {
    blocks::table.limit(limit).order(blocks::slot).into_boxed()
}
