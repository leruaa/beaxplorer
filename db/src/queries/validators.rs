use diesel::{PgConnection, QueryDsl, QueryResult, RunQueryDsl};

use crate::{models::ValidatorModel, utils::pagination::*};

use super::super::schema::validators::dsl::*;

pub fn by_number<'a>(number: i32, connection: &PgConnection) -> QueryResult<ValidatorModel> {
    validators.find(number).first(connection)
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
