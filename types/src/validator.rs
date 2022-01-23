use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
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

pub type ValidatorModelWithId = (u64, ValidatorModel);

#[derive(Serialize, Debug, Clone)]
pub struct ValidatorView {
    pub validator_index: u64,
    #[serde(flatten)]
    pub model: ValidatorModel,
}

impl From<(u64, ValidatorModel)> for ValidatorView {
    fn from((validator_index, model): (u64, ValidatorModel)) -> Self {
        ValidatorView {
            validator_index,
            model,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValidatorsMeta {
    pub count: usize,
}

impl ValidatorsMeta {
    pub fn new(count: usize) -> Self {
        ValidatorsMeta { count }
    }
}
