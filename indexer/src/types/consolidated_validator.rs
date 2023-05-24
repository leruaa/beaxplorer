use std::marker::PhantomData;

use eth2::types::ValidatorStatus;
use lighthouse_types::{Epoch, EthSpec, Validator};
use types::validator::ValidatorModel;

#[derive(Debug)]
pub struct ConsolidatedValidator<'a, E: EthSpec> {
    validator: &'a Validator,
    current_epoch: Epoch,
    balance: &'a u64,
    phantom: PhantomData<E>,
}

impl<'a, E: EthSpec> ConsolidatedValidator<'a, E> {
    pub fn new(validator: &'a Validator, current_epoch: Epoch, balance: &'a u64) -> Self {
        Self {
            validator,
            current_epoch,
            balance,
            phantom: PhantomData::default(),
        }
    }

    pub fn status(&self) -> ValidatorStatus {
        ValidatorStatus::from_validator(
            &self.validator,
            self.current_epoch,
            E::default_spec().far_future_epoch,
        )
    }
}

impl<'a, E: EthSpec> From<ConsolidatedValidator<'a, E>> for ValidatorModel {
    fn from(value: ConsolidatedValidator<E>) -> Self {
        let far_future_epoch = E::default_spec().far_future_epoch;

        ValidatorModel {
            pubkey: value.validator.pubkey.to_string(),
            withdrawal_credentials: format!("{:?}", value.validator.withdrawal_credentials),
            balance: *value.balance,
            effective_balance: value.validator.effective_balance,
            slashed: value.validator.slashed,
            activation_eligibility_epoch: to_epoch_option(
                value.validator.activation_eligibility_epoch,
                far_future_epoch,
            ),
            activation_epoch: value.validator.activation_epoch.as_u64(),
            exit_epoch: to_epoch_option(value.validator.exit_epoch, far_future_epoch),
            withdrawable_epoch: to_epoch_option(
                value.validator.withdrawable_epoch,
                far_future_epoch,
            ),
            status: value.status().to_string(),
        }
    }
}

fn to_epoch_option(epoch: Epoch, far_future_epoch: Epoch) -> Option<u64> {
    if epoch == far_future_epoch {
        None
    } else {
        Some(epoch.as_u64())
    }
}
