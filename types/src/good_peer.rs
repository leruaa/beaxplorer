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
#[persistable(prefix = "/good_peers")]
#[persistable(sortable_field(name = "id", ty = "String", with = "get_id"))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
pub struct GoodPeerModel {
    pub address: String,
}

fn get_id(value: &GoodPeerModelWithId) -> Orderable<String, String> {
    (value.id.clone(), value.id.clone()).into()
}

impl WithMeta for GoodPeerModel {
    type MetaType = GoodPeersMeta;
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[persistable(prefix = "/good_peers/meta")]
pub struct GoodPeersMeta {
    pub count: usize,
}

impl GoodPeersMeta {
    pub fn new(count: usize) -> Self {
        GoodPeersMeta { count }
    }
}

impl Meta for GoodPeersMeta {
    fn count(&self) -> usize {
        self.count
    }
}
