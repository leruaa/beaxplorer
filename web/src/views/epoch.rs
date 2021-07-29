use db::models::EpochModel;
use serde::Serialize;

use crate::helpers::to_formatted_string::{ToEther, ToFormattedString, ToPercentage};

#[derive(Serialize, Default)]
pub struct EpochView {
    pub epoch: String,
    pub attestations_count: String,
    pub deposits_count: String,
    pub eligible_ether: String,
    pub voted_ether: String,
    pub global_participation_percentage: String,
}

impl From<EpochModel> for EpochView {
    fn from(model: EpochModel) -> Self {
        EpochView {
            epoch: model.epoch.to_string(),
            attestations_count: model.attestations_count.to_string(),
            deposits_count: model.deposits_count.to_string(),
            eligible_ether: model.eligible_ether.to_ether_value(),
            voted_ether: model.voted_ether.to_ether_value(),
            global_participation_percentage: model.global_participation_rate.to_percentage(),
        }
    }
}
