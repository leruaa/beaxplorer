use diesel::{pg::Pg, ExpressionMethods, QueryDsl};

use crate::{models::BlockModel, utils::pagination::*};

use super::super::schema::blocks::dsl::*;
pub use diesel::prelude::*;

pub fn by_slot<'a>(s: i64, connection: &PgConnection) -> QueryResult<BlockModel> {
    blocks.limit(1).filter(slot.eq_all(s)).first(connection)
}

pub fn get_paginated<'a>(
    page: i64,
    connection: &PgConnection,
) -> QueryResult<(Vec<BlockModel>, i64)> {
    blocks
        .order(slot)
        .paginate(page)
        .load_and_count_pages(connection)
}
