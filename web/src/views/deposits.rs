use serde::Serialize;
use tsify::Tsify;
use types::deposit::ExecutionLayerDepositModel;
use wasm_bindgen::JsValue;

use crate::to_js;

#[derive(Serialize, Tsify, Debug, Clone)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionLayerDepositView {
    pub index: u64,
    #[serde(flatten)]
    pub model: ExecutionLayerDepositModel,
}

impl From<(u64, ExecutionLayerDepositModel)> for ExecutionLayerDepositView {
    fn from((index, model): (u64, ExecutionLayerDepositModel)) -> Self {
        ExecutionLayerDepositView { index, model }
    }
}

impl From<ExecutionLayerDepositView> for JsValue {
    fn from(val: ExecutionLayerDepositView) -> Self {
        to_js(&val).unwrap()
    }
}

#[derive(Serialize, Tsify, Debug, Clone)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct DepositPaths {
    pub el_deposit: String,
}
