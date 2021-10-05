use diesel::{PgConnection, QueryDsl, QueryResult, RunQueryDsl};

use crate::models::ValidatorModel;

use super::super::schema::validators;

pub fn by_number<'a>(number: i32, connection: &PgConnection) -> QueryResult<ValidatorModel> {
    validators::table.find(number).first(connection)
}

pub fn get_latests<'a>(limit: i64, connection: &PgConnection) -> QueryResult<Vec<ValidatorModel>> {
    validators::table
        .limit(limit)
        .order(validators::validator_index)
        .load(connection)
}
