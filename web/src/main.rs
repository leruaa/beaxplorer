#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use crate::contexts::home::HomeContext;
use contexts::block::BlockContext;
use contexts::blocks::BlocksContext;
use contexts::epoch::EpochContext;
use contexts::epochs::EpochsContext;
use contexts::validator::ValidatorContext;
use contexts::validators::ValidatorsContext;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;

use rocket_sync_db_pools::database;
use types::MainnetEthSpec;

pub mod contexts;
pub mod helpers;
pub mod views;

#[database("node")]
struct NodeDbConn(db::PgConnection);

#[get("/")]
async fn index(db_connection: NodeDbConn) -> Template {
    db_connection
        .run(|c| {
            let epochs = db::queries::epochs::get_latests(10, c).unwrap();
            Template::render("index", HomeContext::<MainnetEthSpec>::new(epochs))
        })
        .await
}

#[get("/epochs?<page>")]
async fn epochs(page: Option<i64>, db_connection: NodeDbConn) -> Template {
    db_connection
        .run(move |c| {
            let epochs = db::queries::epochs::get_paginated(page.unwrap_or_else(|| 1), c).unwrap();
            Template::render("epochs", EpochsContext::<MainnetEthSpec>::new(epochs.0))
        })
        .await
}

#[get("/epoch/<number>")]
async fn epoch(number: i64, db_connection: NodeDbConn) -> Template {
    db_connection
        .run(move |c| {
            let epoch = db::queries::epochs::by_number(number, c).unwrap();
            Template::render("epoch", EpochContext::<MainnetEthSpec>::new(epoch))
        })
        .await
}

#[get("/blocks?<page>")]
async fn blocks(page: Option<i64>, db_connection: NodeDbConn) -> Template {
    db_connection
        .run(move |c| -> Template {
            let blocks = db::queries::blocks::get_paginated(page.unwrap_or_else(|| 1), &c).unwrap();
            Template::render("blocks", BlocksContext::<MainnetEthSpec>::new(blocks.0))
        })
        .await
}

#[get("/block/<slot>")]
async fn block(slot: i64, db_connection: NodeDbConn) -> Template {
    db_connection
        .run(move |c| {
            let block = db::queries::blocks::by_slot(slot, &c).unwrap();
            Template::render("block", BlockContext::<MainnetEthSpec>::new(block))
        })
        .await
}

#[get("/validators?<page>")]
async fn validators(page: Option<i64>, db_connection: NodeDbConn) -> Template {
    db_connection
        .run(move |c| -> Template {
            let validators =
                db::queries::validators::get_paginated(page.unwrap_or_else(|| 1), &c).unwrap();
            Template::render(
                "validators",
                ValidatorsContext::<MainnetEthSpec>::new(validators.0),
            )
        })
        .await
}

#[get("/validator/<number>")]
async fn validator(number: i32, db_connection: NodeDbConn) -> Template {
    db_connection
        .run(move |c| {
            let validator = db::queries::validators::by_number(number, &c).unwrap();
            Template::render(
                "validator",
                ValidatorContext::<MainnetEthSpec>::new(validator),
            )
        })
        .await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![index, epochs, epoch, blocks, block, validators, validator],
        )
        .mount("/static", FileServer::from(relative!("frontend/dist")))
        .attach(Template::fairing())
        .attach(NodeDbConn::fairing())
}
