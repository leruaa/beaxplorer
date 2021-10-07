#[macro_use]
extern crate diesel;

pub mod models;
pub mod queries;
pub mod schema;
pub mod utils;

pub use diesel::insert_into;
pub use diesel::prelude::*;
pub use diesel::r2d2::*;
pub use diesel::result::Error as DieselError;
