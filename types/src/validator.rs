use indexer_macro::Persistable;
use indexer_macro::ToPath;
use indexer_macro::ToPathWithId;
use serde::Deserialize;
use serde::Serialize;

use crate::model::ModelWithId;

#[derive(Persistable, ToPathWithId, Serialize, Deserialize, Debug, Clone)]
#[persistable(index = "model")]
#[to_path(prefix = "/validators")]
pub struct ValidatorModel {
    pub pubkey: Vec<u8>,
    pub pubkey_hex: String,
    pub withdrawable_epoch: Option<u64>,
    pub withdrawal_credentials: Vec<u8>,
    pub balance: u64,
    pub balance_activation: u64,
    pub effective_balance: u64,
    pub slashed: bool,
    pub activation_eligibility_epoch: Option<u64>,
    pub activation_epoch: u64,
    pub exit_epoch: Option<u64>,
    pub status: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct ValidatorView {
    pub validator_index: u64,
    #[serde(flatten)]
    pub model: ValidatorModel,
}

impl From<ValidatorModelWithId> for ValidatorView {
    fn from(value: ValidatorModelWithId) -> Self {
        ValidatorView {
            validator_index: value.id,
            model: value.model,
        }
    }
}

#[derive(ToPath, Serialize, Deserialize, Debug, Clone)]
#[to_path(prefix = "/validators/meta")]
pub struct ValidatorsMeta {
    pub count: usize,
}

impl ValidatorsMeta {
    pub fn new(count: usize) -> Self {
        ValidatorsMeta { count }
    }
}
