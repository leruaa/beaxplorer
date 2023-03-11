use indexer_macro::{Persistable, ToPath, ToPathWithId};
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    meta::{Meta, WithMeta},
    model::ModelWithId,
};

#[derive(Persistable, ToPathWithId, Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[persistable(index = "model")]
#[to_path(prefix = "/block_requests")]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(rename_all = "camelCase")]
pub struct BlockRequestModel {
    pub root: String,
    pub failed_count: u64,
    pub not_found_count: u64,
    pub state: String,
}

impl WithMeta for BlockRequestModel {
    type MetaType = BlockRequestsMeta;
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(ToPath, Serialize, Deserialize, Debug, Clone)]
#[to_path(prefix = "/block_requests/meta")]
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
