use crate::schema::epochs;
use crate::schema::blocks;

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

#[derive(Queryable, Insertable)]
#[table_name = "blocks"]
pub struct Block {
    pub epoch: i64,
    pub slot: i64,
    pub block_root: Vec<u8>,
    pub parent_root: Vec<u8>,
    pub state_root: Vec<u8>,
    pub signature: Vec<u8>,
    pub randao_reveal: Option<Vec<u8>>,
    pub graffiti: Option<Vec<u8>>,
    pub graffiti_text: Option<String>,
    pub eth1data_deposit_root: Option<Vec<u8>>,
    pub eth1data_deposit_count: i32,
    pub eth1data_block_hash: Option<Vec<u8>>,
    pub proposer_slashings_count: i32,
    pub attester_slashings_count: i32,
    pub attestations_count: i32,
    pub deposits_count: i32,
    pub voluntary_exits_count: i32,
    pub proposer: i32,
    pub status: String,
}