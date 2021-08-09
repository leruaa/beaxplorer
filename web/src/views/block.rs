use db::models::BlockModel;
use serde::Serialize;
use shared::utils::clock::Clock;
use std::{
    convert::{TryFrom, TryInto},
    marker::PhantomData,
};
use types::{EthSpec, Slot};

use super::errors::ConversionError;

#[derive(Serialize, Default)]
pub struct BlockView<E: EthSpec> {
    pub epoch: String,
    pub slot: String,
    pub proposer: String,
    pub attestations_count: String,
    pub timestamp: u64,
    phantom: PhantomData<E>,
}

impl<E: EthSpec> TryFrom<BlockModel> for BlockView<E> {
    type Error = ConversionError;

    fn try_from(model: BlockModel) -> Result<Self, Self::Error> {
        let slot = Slot::new(model.slot.try_into()?);
        let spec = E::default_spec();
        let clock = Clock::new(spec);
        let view = BlockView {
            epoch: model.epoch.to_string(),
            slot: model.slot.to_string(),
            proposer: model.proposer.to_string(),
            attestations_count: model.attestations_count.to_string(),
            timestamp: clock
                .start_of(slot)
                .ok_or(ConversionError::SlotNotFound)?
                .as_secs(),
            phantom: PhantomData,
        };

        Ok(view)
    }
}
