use std::{
    convert::{TryFrom, TryInto},
    marker::PhantomData,
};

use db::models::EpochModel;
use serde::Serialize;
use shared::utils::clock::Clock;
use types::{Epoch, EthSpec};

use crate::helpers::to_formatted_string::{ToEther, ToPercentage};

use super::errors::ConversionError;

#[derive(Serialize, Default)]
pub struct EpochView<E: EthSpec> {
    pub epoch: String,
    pub attestations_count: String,
    pub deposits_count: String,
    pub proposer_slashings_count: String,
    pub attester_slashings_count: String,
    pub eligible_ether: String,
    pub voted_ether: String,
    pub global_participation_percentage: String,
    pub finalized: bool,
    pub time: String,
    pub ago: String,
    phantom: PhantomData<E>,
}

impl<E: EthSpec> TryFrom<EpochModel> for EpochView<E> {
    type Error = ConversionError;

    fn try_from(model: EpochModel) -> Result<Self, Self::Error> {
        let epoch = Epoch::new(model.epoch.try_into()?);
        let start_slot = epoch.start_slot(E::slots_per_epoch());
        let spec = E::default_spec();
        let clock = Clock::new(spec);
        let view = EpochView {
            epoch: model.epoch.to_string(),
            attestations_count: model.attestations_count.to_string(),
            deposits_count: model.deposits_count.to_string(),
            eligible_ether: model.eligible_ether.to_ether_value(),
            proposer_slashings_count: model.proposer_slashings_count.to_string(),
            attester_slashings_count: model.attester_slashings_count.to_string(),
            voted_ether: model.voted_ether.to_ether_value(),
            global_participation_percentage: model.global_participation_rate.to_percentage(),
            finalized: model.finalized.unwrap_or_default(),
            time: clock.format(start_slot),
            ago: clock.ago(start_slot),
            phantom: PhantomData,
        };

        Ok(view)
    }
}
