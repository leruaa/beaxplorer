use std::marker::PhantomData;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone)]
pub struct BlockView {
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

#[derive(Serialize, Deserialize, Clone)]
pub struct EpochView {
    pub epoch: i64,
    pub timestamp: u64,
    pub blocks_count: i32,
    pub proposer_slashings_count: i32,
    pub attester_slashings_count: i32,
    pub attestations_count: i32,
    pub deposits_count: i32,
    pub voluntary_exits_count: i32,
    pub validators_count: i32,
    pub average_validator_balance: i64,
    pub total_validator_balance: i64,
    pub finalized: bool,
    pub eligible_ether: Option<String>,
    pub global_participation_rate: Option<String>,
    pub voted_ether: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ValidatorView {
    pub validator_index: i32,
    pub pubkey: Vec<u8>,
    pub pubkey_hex: String,
    pub withdrawable_epoch: i64,
    pub withdrawal_credentials: Vec<u8>,
    pub balance: i64,
    pub balance_activation: Option<i64>,
    pub effective_balance: i64,
    pub slashed: bool,
    pub activation_eligibility_epoch: i64,
    pub activation_epoch: i64,
    pub exit_epoch: i64,
    pub status: String,
}