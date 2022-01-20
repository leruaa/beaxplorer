use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockView {
    pub epoch: u64,
    pub slot: u64,
    pub block_root: Vec<u8>,
    pub parent_root: Vec<u8>,
    pub state_root: Vec<u8>,
    pub signature: Vec<u8>,
    pub randao_reveal: Vec<u8>,
    pub graffiti: Vec<u8>,
    pub graffiti_text: String,
    pub eth1data_deposit_root: Vec<u8>,
    pub eth1data_deposit_count: u64,
    pub eth1data_block_hash: Vec<u8>,
    pub proposer_slashings_count: usize,
    pub attester_slashings_count: usize,
    pub attestations_count: usize,
    pub deposits_count: usize,
    pub voluntary_exits_count: usize,
    pub proposer: u64,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EpochView {
    pub epoch: u64,
    pub timestamp: u64,
    pub blocks_count: usize,
    pub proposer_slashings_count: usize,
    pub attester_slashings_count: usize,
    pub attestations_count: usize,
    pub deposits_count: usize,
    pub voluntary_exits_count: usize,
    pub validators_count: usize,
    pub average_validator_balance: u64,
    pub total_validator_balance: u64,
    pub finalized: bool,
    pub eligible_ether: u64,
    pub global_participation_rate: f64,
    pub voted_ether: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValidatorView {
    pub validator_index: u64,
    pub pubkey: Vec<u8>,
    pub pubkey_hex: String,
    pub withdrawable_epoch: Option<u64>,
    pub withdrawal_credentials: Vec<u8>,
    pub balance: u64,
    pub balance_activation: u64,
    pub effective_balance: u64,
    pub slashed: bool,
    pub activation_eligibility_epoch: Option<u64>,
    pub activation_epoch: u64,
    pub exit_epoch: Option<u64>,
    pub status: String,
}
