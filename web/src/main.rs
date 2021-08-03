#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use crate::contexts::home::HomeContext;
use contexts::epoch::EpochContext;
use contexts::epochs::EpochsContext;
use db::models::EpochModel;
use db::RunQueryDsl;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;

use rocket_sync_db_pools::database;

pub mod contexts;
pub mod helpers;
pub mod views;

#[database("node")]
struct NodeDbConn(db::PgConnection);

#[get("/")]
async fn index(db_connection: NodeDbConn) -> Template {
    db_connection
        .run(|c| {
            let epochs = db::queries::epochs::get_latests(10)
                .load::<EpochModel>(c)
                .unwrap();
            Template::render("index", HomeContext::new(epochs))
        })
        .await
}

#[get("/epochs")]
async fn epochs(db_connection: NodeDbConn) -> Template {
    db_connection
        .run(|c| {
            let epochs = db::queries::epochs::get_latests(10)
                .load::<EpochModel>(c)
                .unwrap();
            Template::render("epochs", EpochsContext::new(epochs))
        })
        .await
}

#[get("/epoch/<number>")]
async fn epoch(number: i64, db_connection: NodeDbConn) -> Template {
    db_connection
        .run(move |c| {
            let epoch = db::queries::epochs::by_number(number).first(c).unwrap();
            Template::render("epoch", EpochContext::new(epoch))
        })
        .await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, epochs, epoch])
        .mount("/static", FileServer::from(relative!("frontend/dist")))
        .attach(Template::fairing())
        .attach(NodeDbConn::fairing())
}
