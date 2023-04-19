use indexer_macro::Persistable;
use serde::Deserialize;
use serde::Serialize;

#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[persistable(model = "default")]
#[persistable(prefix = "/validators")]
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
