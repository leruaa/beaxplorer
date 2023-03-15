use indexer_macro::Persistable;
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    meta::{Meta, WithMeta},
    model::ModelWithId,
    utils::Orderable,
};

#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[persistable(id = "String")]
#[persistable(model = "default")]
#[persistable(prefix = "/block_requests")]
#[persistable(sortable_field(name = "root", ty = "String", with = "get_root"))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(rename_all = "camelCase")]
pub struct BlockRequestModel {
    pub failed_count: u64,
    pub not_found_count: u64,
    pub state: String,
}

fn get_root(value: &BlockRequestModelWithId) -> Orderable<String, String> {
    (value.id.clone(), value.id.clone()).into()
}

impl WithMeta for BlockRequestModel {
    type MetaType = BlockRequestsMeta;
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[persistable(prefix = "/block_requests/meta")]
pub struct BlockRequestsMeta {
    pub count: usize,
}

impl BlockRequestsMeta {
    pub fn new(count: usize) -> Self {
        BlockRequestsMeta { count }
    }
}

impl Meta for BlockRequestsMeta {
    fn count(&self) -> usize {
        self.count
    }
}
