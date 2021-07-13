#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;
pub mod queries;

pub use diesel::prelude::*;
pub use diesel::insert_into;
pub use diesel::result::Error as DieselError;