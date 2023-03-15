use indexer_macro::Persistable;
use serde::Deserialize;
use serde::Serialize;

use crate::meta::Meta;
use crate::meta::WithMeta;
use crate::model::ModelWithId;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

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

impl WithMeta for ValidatorModel {
    type MetaType = ValidatorsMeta;
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[persistable(prefix = "/validators/meta")]
pub struct ValidatorsMeta {
    pub count: usize,
}

impl ValidatorsMeta {
    pub fn new(count: usize) -> Self {
        ValidatorsMeta { count }
    }
}

impl Meta for ValidatorsMeta {
    fn count(&self) -> usize {
        self.count
    }
}
