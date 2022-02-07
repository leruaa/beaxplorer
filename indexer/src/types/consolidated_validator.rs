use eth2::types::{StateId, ValidatorData};
use types::validator::{ValidatorModel, ValidatorModelWithId};

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
}

impl From<&ConsolidatedValidator> for ValidatorModelWithId {
    fn from(value: &ConsolidatedValidator) -> Self {
        let model = ValidatorModel {
            pubkey: value.0.validator.pubkey.as_serialized().to_vec(),
            pubkey_hex: value.0.validator.pubkey.to_string(),
            withdrawable_epoch: match value.0.validator.withdrawable_epoch.as_u64() {
                u64::MAX => None,
                x => Some(x),
            },
            withdrawal_credentials: value.0.validator.withdrawal_credentials.as_bytes().to_vec(),
            balance: value.0.balance,
            balance_activation: value.0.validator.activation_epoch.as_u64(),
            effective_balance: value.0.validator.effective_balance,
            slashed: value.0.validator.slashed,
            activation_eligibility_epoch: match value
                .0
                .validator
                .activation_eligibility_epoch
                .as_u64()
            {
                u64::MAX => None,
                x => Some(x),
            },
            activation_epoch: value.0.validator.activation_epoch.as_u64(),
            exit_epoch: match value.0.validator.exit_epoch.as_u64() {
                u64::MAX => None,
                x => Some(x),
            },
            status: value.0.status.to_string(),
        };

        ValidatorModelWithId {
            id: value.0.index,
            model,
        }
    }
}
