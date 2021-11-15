use db::{
    models::BlockModel, schema::blocks::columns, utils::pagination::Paginate, PgConnection,
    QueryDsl, QueryResult,
};

pub fn get_paginated<'a>(
    page: i64,
    connection: &PgConnection,
) -> QueryResult<(Vec<BlockModel>, i64)> {
    db::queries::blocks::all()
        .order_by(columns::slot)
        .paginate(page)
        .load_and_count_pages(connection)
}
