use db::{
    models::EpochModel, schema::epochs::columns, utils::pagination::Paginate, ExpressionMethods,
    PgConnection, QueryDsl, QueryResult,
};

pub fn get_paginated<'a>(
    page: i64,
    sort: Option<String>,
    direction: Option<String>,
    connection: &PgConnection,
) -> QueryResult<(Vec<EpochModel>, i64)> {
    let mut query = db::queries::epochs::all();

    query = match (
        sort.as_ref().map_or("epoch", String::as_str),
        direction.as_ref().map_or("asc", String::as_str),
    ) {
        ("epoch", "desc") | ("ago", "desc") => query.order_by(columns::epoch.desc()),

        ("attestations_count", "desc") => query.order_by(columns::attestations_count.desc()),
        ("attestations_count", _) => query.order_by(columns::attestations_count.asc()),

        ("deposits_count", "desc") => query.order_by(columns::deposits_count.desc()),
        ("deposits_count", _) => query.order_by(columns::deposits_count.asc()),

        ("proposer_slashings_count", "desc") => {
            query.order_by(columns::proposer_slashings_count.desc())
        }
        ("proposer_slashings_count", _) => query.order_by(columns::proposer_slashings_count.asc()),

        ("attester_slashings_count", "desc") => {
            query.order_by(columns::attester_slashings_count.desc())
        }
        ("attester_slashings_count", _) => query.order_by(columns::attester_slashings_count.asc()),

        ("finalized", "desc") => query.order_by(columns::finalized.desc()),
        ("finalized", _) => query.order_by(columns::finalized.asc()),

        ("eligible_ether", "desc") => query.order_by(columns::eligible_ether.desc()),
        ("eligible_ether", _) => query.order_by(columns::eligible_ether.asc()),

        ("voted_ether", "desc") => query.order_by(columns::voted_ether.desc()),
        ("voted_ether", _) => query.order_by(columns::voted_ether.asc()),

        ("global_participation_rate", "desc") => {
            query.order_by(columns::global_participation_rate.desc())
        }
        ("global_participation_rate", _) => {
            query.order_by(columns::global_participation_rate.asc())
        }

        (_, _) => query.order_by(columns::epoch.asc()),
    };

    query = query.then_order_by(columns::epoch.asc());

    query.paginate(page).load_and_count_pages(connection)
}
