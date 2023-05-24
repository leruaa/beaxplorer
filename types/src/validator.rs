use indexer_macro::Persistable;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
#[cfg(feature = "wasm")]
use tsify::Tsify;

#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[persistable(model = "default")]
#[persistable(prefix = "/validators")]
#[serde(rename_all = "camelCase")]
pub struct ValidatorModel {
    pub pubkey: String,
    pub withdrawal_credentials: String,
    pub balance: u64,
    pub effective_balance: u64,
    pub slashed: bool,
    pub activation_eligibility_epoch: Option<u64>,
    pub activation_epoch: u64,
    pub exit_epoch: Option<u64>,
    pub withdrawable_epoch: Option<u64>,
    pub status: String,
}

#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[persistable(model = "default")]
#[persistable(prefix = "/validators/e")]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(rename_all = "camelCase")]

pub struct ValidatorExtendedModel {
    pub execution_layer_deposits: HashSet<u64>,
}
