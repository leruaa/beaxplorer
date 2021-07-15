#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use db::models::EpochModel;
use db::RunQueryDsl;
use rocket::fs::{FileServer, relative};
use serde::Serialize;
use rocket_dyn_templates::Template;

use rocket_sync_db_pools::database;

#[database("node")]
struct NodeDbConn(db::PgConnection);

#[derive(Serialize)]
struct HomeContext {
    pub epochs: Vec<EpochModel>
}

#[get("/")]
async fn index(db_connection: NodeDbConn) -> Template {
    db_connection.run(|c| {
        let epochs = db::queries::epochs::get_latests(10).load::<EpochModel>(c).unwrap();
        Template::render("index", HomeContext { epochs })
    }).await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/static", FileServer::from(relative!("frontend/dist")))
        .attach(Template::fairing())
        .attach(NodeDbConn::fairing())
}