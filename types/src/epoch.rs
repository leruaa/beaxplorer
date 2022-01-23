use std::ops::Div;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EpochModel {
    pub timestamp: u64,
    pub proposer_slashings_count: usize,
    pub attester_slashings_count: usize,
    pub attestations_count: usize,
    pub deposits_count: usize,
    pub eligible_ether: u64,
    pub voted_ether: u64,
}

pub type EpochModelWithId = (u64, EpochModel);

#[derive(Serialize, Debug, Clone)]
pub struct EpochView {
    pub epoch: u64,
    #[serde(flatten)]
    pub model: EpochModel,
    pub finalized: bool,
    pub global_participation_rate: f64,
}

impl From<(u64, EpochModel)> for EpochView {
    fn from((epoch, model): (u64, EpochModel)) -> Self {
        let global_participation_rate = (model.voted_ether as f64).div(model.eligible_ether as f64);

        EpochView {
            epoch,
            model,
            finalized: global_participation_rate >= 2f64 / 3f64,
            global_participation_rate: global_participation_rate,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EpochExtendedModel {
    pub voluntary_exits_count: usize,
    pub validators_count: usize,
    pub average_validator_balance: u64,
    pub total_validator_balance: u64,
}

pub type EpochExtendedModelWithId = (u64, EpochExtendedModel);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EpochsMeta {
    pub count: usize,
}

impl EpochsMeta {
    pub fn new(count: usize) -> Self {
        EpochsMeta { count }
    }
}
