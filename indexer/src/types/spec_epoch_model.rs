use std::{
    convert::{TryFrom, TryInto},
    marker::PhantomData,
};

use db::models::EpochModel;
use lighthouse_types::{Epoch, EthSpec};
use shared::{errors::ConversionError, utils::clock::Clock};
use types::views::EpochView;

pub struct SpecEpochModel<'a, E: EthSpec> {
    pub inner: &'a EpochModel,
    phantom: PhantomData<E>,
}

impl<'a, E: EthSpec> SpecEpochModel<'a, E> {
    pub fn new(inner: &'a EpochModel) -> Self {
        SpecEpochModel {
            inner,
            phantom: PhantomData,
        }
    }
}

impl<'a, E: EthSpec> TryFrom<SpecEpochModel<'a, E>> for EpochView {
    type Error = ConversionError;

    fn try_from(model: SpecEpochModel<E>) -> Result<Self, Self::Error> {
        let epoch = Epoch::new(model.inner.epoch.try_into()?);
        let start_slot = epoch.start_slot(E::slots_per_epoch());
        let spec = E::default_spec();
        let clock = Clock::new(spec);
        let view = EpochView {
            epoch: model.inner.epoch,
            timestamp: clock.timestamp(start_slot).unwrap_or(0),
            blocks_count: model.inner.blocks_count,
            proposer_slashings_count: model.inner.proposer_slashings_count,
            attester_slashings_count: model.inner.attester_slashings_count,
            attestations_count: model.inner.attestations_count,
            deposits_count: model.inner.deposits_count,
            voluntary_exits_count: model.inner.voluntary_exits_count,
            validators_count: model.inner.validators_count,
            average_validator_balance: model.inner.average_validator_balance,
            total_validator_balance: model.inner.total_validator_balance,
            finalized: model.inner.finalized.unwrap_or(false),
            eligible_ether: model.inner.eligible_ether.map(|x| x.to_string()),
            global_participation_rate: model.inner.global_participation_rate.map(|x| x.to_string()),
            voted_ether: model.inner.voted_ether.map(|x| x.to_string()),
        };

        Ok(view)
    }
}
