use std::convert::TryInto;

use crate::{
    controllers,
    helpers::db::NodeDbConn,
    views::{
        block::BlockView, epoch::EpochView, paginated::PaginatedView, validator::ValidatorView,
    },
};
use rocket::serde::json::Json;
use types::MainnetEthSpec;

#[get("/epochs?<sort>&<page>&<dir>")]
pub async fn epochs(
    page: Option<i64>,
    sort: Option<String>,
    dir: Option<String>,
    db_connection: NodeDbConn,
) -> Json<PaginatedView<EpochView<MainnetEthSpec>>> {
    db_connection
        .run(move |c| {
            let paginated =
                controllers::epochs::get_paginated(page.unwrap_or_else(|| 1), sort, dir, c)
                    .unwrap();

            let epochs = paginated
                .0
                .into_iter()
                .map(|e| e.try_into().ok())
                .filter_map(|e| e)
                .collect();

            Json(PaginatedView::new(epochs, paginated.1))
        })
        .await
}

#[get("/blocks?<sort>&<page>&<dir>")]
pub async fn blocks(
    page: Option<i64>,
    sort: Option<String>,
    dir: Option<String>,
    db_connection: NodeDbConn,
) -> Json<PaginatedView<BlockView<MainnetEthSpec>>> {
    db_connection
        .run(move |c| {
            let paginated =
                controllers::blocks::get_paginated(page.unwrap_or_else(|| 1), sort, dir, c)
                    .unwrap();

            let blocks = paginated
                .0
                .into_iter()
                .map(|e| e.try_into().ok())
                .filter_map(|e| e)
                .collect();

            Json(PaginatedView::new(blocks, paginated.1))
        })
        .await
}

#[get("/validators?<sort>&<page>&<dir>")]
pub async fn validators(
    page: Option<i64>,
    sort: Option<String>,
    dir: Option<String>,
    db_connection: NodeDbConn,
) -> Json<PaginatedView<ValidatorView<MainnetEthSpec>>> {
    db_connection
        .run(move |c| {
            let paginated =
                controllers::validators::get_paginated(page.unwrap_or_else(|| 1), sort, dir, c)
                    .unwrap();

            let validators = paginated
                .0
                .into_iter()
                .map(|e| e.try_into().ok())
                .filter_map(|e| e)
                .collect();

            Json(PaginatedView::new(validators, paginated.1))
        })
        .await
}
