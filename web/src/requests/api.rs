use crate::helpers::db::NodeDbConn;
use db::models::EpochModel;
use rocket::serde::json::Json;

#[get("/epochs?<page>")]
pub async fn epochs(page: Option<i64>, db_connection: NodeDbConn) -> Json<Vec<EpochModel>> {
    db_connection
        .run(move |c| {
            let epochs = db::queries::epochs::get_paginated(page.unwrap_or_else(|| 1), c).unwrap();

            Json(epochs.0)
        })
        .await
}
