use crate::schema::epochs;

#[derive(Queryable, Insertable)]
#[table_name = "epochs"]
pub struct Epoch {
    pub epoch: i64,
    pub blocks_count: i32,
    pub proposer_slashings_count: i32,
    pub attester_slashings_count: i32,
    pub attestations_count: i32,
    pub deposits_count: i32,
    pub voluntary_exits_count: i32,
    pub validators_count: i32,
    pub average_validator_balance: i64,
    pub total_validator_balance: i64,
    pub finalized: Option<bool>,
    pub eligible_ether: Option<i64>,
    pub global_participation_rate: Option<f64>,
    pub voted_ether: Option<i64>,
}