use diesel::{
    dsl::max,
    pg::Pg,
    sql_types::{BigInt, Nullable},
    QueryDsl,
};

use crate::schema::epochs::dsl::*;
use diesel::prelude::*;

type EpochsBoxedQuery<'a> = crate::schema::epochs::BoxedQuery<'a, Pg>;

pub fn all<'a>() -> EpochsBoxedQuery<'a> {
    epochs.into_boxed()
}

pub fn by_number<'a>(e: i64) -> EpochsBoxedQuery<'a> {
    epochs.find(e).into_boxed()
}

pub fn get_latests<'a>(limit: i64) -> EpochsBoxedQuery<'a> {
    epochs.limit(limit).order(epoch).into_boxed()
}

pub fn get_latest_finalized_epoch<'a>(
) -> crate::schema::epochs::BoxedQuery<'a, Pg, Nullable<BigInt>> {
    epochs
        .select(max(epoch))
        .filter(finalized.eq_all(true))
        .into_boxed()
}
