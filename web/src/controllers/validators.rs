use db::{
    models::ValidatorModel, schema::validators::columns, utils::pagination::Paginate, PgConnection,
    QueryDsl, QueryResult,
};

pub fn get_paginated<'a>(
    page: i64,
    connection: &PgConnection,
) -> QueryResult<(Vec<ValidatorModel>, i64)> {
    db::queries::validators::all()
        .order_by(columns::validator_index)
        .paginate(page)
        .load_and_count_pages(connection)
}
