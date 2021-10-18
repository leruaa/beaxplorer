use std::convert::TryInto;

use crate::{helpers::db::NodeDbConn, views::epoch::EpochView};
use rocket::serde::json::Json;
use types::MainnetEthSpec;

#[get("/epochs?<page>")]
pub async fn epochs(
    page: Option<i64>,
    db_connection: NodeDbConn,
) -> Json<Vec<EpochView<MainnetEthSpec>>> {
    db_connection
        .run(move |c| {
            let epochs = db::queries::epochs::get_paginated(page.unwrap_or_else(|| 1), c)
                .unwrap()
                .0
                .into_iter()
                .map(|e| e.try_into().ok())
                .filter_map(|e| e)
                .collect();

            Json(epochs)
        })
        .await
}