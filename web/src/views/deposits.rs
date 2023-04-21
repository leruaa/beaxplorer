use serde::Serialize;
use tsify::Tsify;
use types::deposit::DepositModel;
use wasm_bindgen::JsValue;

use crate::to_js;

#[derive(Serialize, Tsify, Debug, Clone)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct DepositView {
    #[serde(flatten)]
    pub model: DepositModel,
}

impl From<DepositModel> for DepositView {
    fn from(model: DepositModel) -> Self {
        DepositView { model }
    }
}

impl From<DepositView> for JsValue {
    fn from(val: DepositView) -> Self {
        to_js(&val).unwrap()
    }
}
