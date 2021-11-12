use diesel::{pg::Pg, PgConnection, QueryDsl, QueryResult};

use crate::{models::ValidatorModel, utils::pagination::*};

use super::super::schema::validators::dsl::*;

type BoxedQuery<'a> = crate::schema::validators::BoxedQuery<'a, Pg>;

pub fn by_number<'a>(number: i32) -> BoxedQuery<'a> {
    validators.find(number).into_boxed()
}

pub fn get_paginated<'a>(
    page: i64,
    connection: &PgConnection,
) -> QueryResult<(Vec<ValidatorModel>, i64)> {
    validators
        .order(validator_index)
        .paginate(page)
        .load_and_count_pages(connection)
}
