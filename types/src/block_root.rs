use crate::model::ModelWithId;
use indexer_macro::Persistable;
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[persistable(id = "String")]
#[persistable(model = "option")]
#[persistable(prefix = "/blocks/root")]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
pub struct BlockRootModel {
    pub slot: u64,
}
