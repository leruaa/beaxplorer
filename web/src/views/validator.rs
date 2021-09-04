use std::{
    convert::{TryFrom, TryInto},
    marker::PhantomData,
};

use db::models::ValidatorModel;
use serde::Serialize;
use shared::utils::clock::Clock;
use types::{Epoch, EthSpec};

use crate::helpers::to_formatted_string::ToEther;

use super::errors::ConversionError;

#[derive(Serialize, Default)]
pub struct ValidatorView<E: EthSpec> {
    pub index: String,
    pub public_key_extract: String,
    pub balance: String,
    pub effective_balance: String,
    pub status: String,
    pub activation_ago: String,
    pub activation_epoch: String,
    pub exit_ago: Option<String>,
    pub exit_epoch: String,
    pub withdrawable_ago: Option<String>,
    pub withdrawable_epoch: String,
    phantom: PhantomData<E>,
}

impl<E: EthSpec> TryFrom<ValidatorModel> for ValidatorView<E> {
    type Error = ConversionError;

    fn try_from(model: ValidatorModel) -> Result<Self, Self::Error> {
        let spec = E::default_spec();
        let clock = Clock::new(spec);
        let activation_epoch = Epoch::new(model.activation_epoch.try_into()?);
        let activation_epoch_start_slot = activation_epoch.start_slot(E::slots_per_epoch());
        let exit_epoch = Epoch::new(model.exit_epoch.try_into()?);
        let exit_epoch_start_slot = exit_epoch.start_slot(E::slots_per_epoch());
        let withdrawable_epoch = Epoch::new(model.withdrawable_epoch.try_into()?);
        let withdrawable_epoch_start_slot = withdrawable_epoch.start_slot(E::slots_per_epoch());

        let view = ValidatorView {
            index: model.validator_index.to_string(),
            public_key_extract: model.pubkey_hex.chars().take(10).collect(),
            balance: model.balance.to_ether_value(4),
            effective_balance: model.effective_balance.to_ether_value(4),
            status: model.status,
            activation_ago: clock.ago(activation_epoch_start_slot),
            activation_epoch: model.activation_epoch.to_string(),
            exit_ago: if exit_epoch == Epoch::max_value() {
                None
            } else {
                Some(clock.ago(exit_epoch_start_slot))
            },
            exit_epoch: model.exit_epoch.to_string(),
            withdrawable_ago: if withdrawable_epoch == Epoch::max_value() {
                None
            } else {
                Some(clock.ago(withdrawable_epoch_start_slot))
            },
            withdrawable_epoch: model.withdrawable_epoch.to_string(),
            phantom: PhantomData,
        };

        Ok(view)
    }
}
