use std::convert::TryInto;

use db::models::ValidatorModel;
use eth2::types::{StateId, ValidatorData};
use shared::utils::convert::{IntoClampedI32, IntoClampedI64};

use crate::{beacon_node_client::BeaconNodeClient, errors::IndexerError};

#[derive(Debug)]
pub struct ConsolidatedValidator(pub ValidatorData);

impl ConsolidatedValidator {
    pub async fn from_state(
        state: StateId,
        client: BeaconNodeClient,
    ) -> Result<Vec<Self>, IndexerError> {
        client.get_validators(state).await.map(|validators| {
            validators
                .into_iter()
                .map(|v| ConsolidatedValidator(v))
                .collect()
        })
    }

    pub fn as_model(&self) -> Result<ValidatorModel, IndexerError> {
        let model = ValidatorModel {
            validator_index: self.0.index.into_i32(),
            pubkey: self.0.validator.pubkey.as_serialized().to_vec(),
            pubkey_hex: self.0.validator.pubkey.to_string(),
            withdrawable_epoch: self.0.validator.withdrawable_epoch.as_u64().into_i64(),
            withdrawal_credentials: self.0.validator.withdrawal_credentials.as_bytes().to_vec(),
            balance: self.0.balance.into_i64(),
            balance_activation: self.0.validator.activation_epoch.as_u64().try_into().ok(),
            effective_balance: self.0.validator.effective_balance.into_i64(),
            slashed: self.0.validator.slashed,
            activation_eligibility_epoch: self
                .0
                .validator
                .activation_eligibility_epoch
                .as_u64()
                .into_i64(),
            activation_epoch: self.0.validator.activation_epoch.as_u64().into_i64(),
            exit_epoch: self.0.validator.exit_epoch.as_u64().into_i64(),
            status: self.0.status.to_string(),
        };

        Ok(model)
    }
}
