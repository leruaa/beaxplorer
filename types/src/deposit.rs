use indexer_macro::Persistable;
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[persistable(model = "default")]
#[persistable(prefix = "/deposits")]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(rename_all = "camelCase")]
pub struct DepositModel {
    pub slot: u64,
    pub public_key: String,
    pub withdrawal_credentials: Vec<u8>,
    pub amount: u64,
    pub signature: String,
}

