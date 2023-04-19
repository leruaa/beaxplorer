use indexer_macro::Persistable;
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::utils::Orderable;

#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[persistable(id = "String")]
#[persistable(model = "default")]
#[persistable(prefix = "/block_requests")]
#[persistable(sortable_field(name = "root", ty = "String", with = "get_root"))]
#[persistable(sortable_field(name = "possible_slots", ty = "u64", with = "get_possible_slot"))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(rename_all = "camelCase")]
pub struct BlockRequestModel {
    pub possible_slots: Vec<u64>,
    pub state: String,
    pub active_request_count: usize,
    pub failed_count: usize,
    pub not_found_count: usize,
    pub found_by: String,
}

fn get_root(value: &BlockRequestModelWithId) -> Orderable<String, String> {
    (value.id.clone(), value.id.clone()).into()
}

fn get_possible_slot(value: &BlockRequestModelWithId) -> Orderable<String, u64> {
    (
        value.id.clone(),
        value.model.possible_slots.first().cloned().unwrap_or(0),
    )
        .into()
}
