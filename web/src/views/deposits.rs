use serde::Serialize;
use tsify::Tsify;
use types::deposit::ExecutionLayerDepositModel;
use wasm_bindgen::JsValue;

use crate::to_js;

#[derive(Serialize, Tsify, Debug, Clone)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionLayerDepositView {
    #[serde(flatten)]
    pub model: ExecutionLayerDepositModel,
}

impl From<ExecutionLayerDepositModel> for ExecutionLayerDepositView {
    fn from(model: ExecutionLayerDepositModel) -> Self {
        ExecutionLayerDepositView { model }
    }
}

impl From<ExecutionLayerDepositView> for JsValue {
    fn from(val: ExecutionLayerDepositView) -> Self {
        to_js(&val).unwrap()
    }
}
