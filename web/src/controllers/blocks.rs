use db::{
    models::BlockModel, schema::blocks::columns, utils::pagination::Paginate, PgConnection,
    QueryDsl, QueryResult,
};

pub fn get_paginated<'a>(
    page: i64,
    sort: Option<String>,
    direction: Option<String>,
    connection: &PgConnection,
) -> QueryResult<(Vec<BlockModel>, i64)> {
    let mut query = db::queries::blocks::all();

    query
        .order_by(columns::slot)
        .paginate(page)
        .load_and_count_pages(connection)
}
