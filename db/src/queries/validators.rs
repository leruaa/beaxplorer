use diesel::{pg::Pg, QueryDsl};

use super::super::schema::validators::dsl::*;

type BoxedQuery<'a> = crate::schema::validators::BoxedQuery<'a, Pg>;

pub fn all<'a>() -> BoxedQuery<'a> {
    validators.into_boxed()
}

pub fn by_number<'a>(number: i32) -> BoxedQuery<'a> {
    validators.find(number).into_boxed()
}
