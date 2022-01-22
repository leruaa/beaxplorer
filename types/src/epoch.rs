use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EpochModel {
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
pub struct EpochsMeta {
    pub count: usize,
}

impl EpochsMeta {
    pub fn new(count: usize) -> Self {
        EpochsMeta { count }
    }
}
