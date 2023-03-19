use serde::Serialize;
use tsify::Tsify;
use types::good_peer::GoodPeerModel;
use wasm_bindgen::JsValue;

use crate::to_js;

#[derive(Serialize, Tsify, Debug, Clone)]
#[tsify(into_wasm_abi)]
pub struct GoodPeerView {
    pub id: String,
    #[serde(flatten)]
    pub model: GoodPeerModel,
}

impl From<(String, GoodPeerModel)> for GoodPeerView {
    fn from((id, model): (String, GoodPeerModel)) -> Self {
        GoodPeerView { id, model }
    }
}

impl From<GoodPeerView> for JsValue {
    fn from(val: GoodPeerView) -> Self {
        to_js(&val).unwrap()
    }
}
