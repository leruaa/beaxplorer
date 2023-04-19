use serde::Serialize;
use tsify::Tsify;
use types::meta::Meta;
use wasm_bindgen::JsValue;

use crate::to_js;

#[derive(Serialize, Tsify, Debug, Clone)]
#[tsify(into_wasm_abi)]
pub struct MetaView {
    #[serde(flatten)]
    pub meta: Meta,
}

impl From<Meta> for MetaView {
    fn from(meta: Meta) -> Self {
        MetaView { meta }
    }
}

impl From<MetaView> for JsValue {
    fn from(val: MetaView) -> Self {
        to_js(&val).unwrap()
    }
}
