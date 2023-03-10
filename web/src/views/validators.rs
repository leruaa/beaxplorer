use serde::Serialize;
use tsify::Tsify;
use types::validator::ValidatorModel;
use wasm_bindgen::JsValue;

use crate::to_js;

#[derive(Serialize, Tsify, Debug, Clone)]
#[tsify(into_wasm_abi)]
pub struct ValidatorView {
    pub validator_index: u64,
    #[serde(flatten)]
    pub model: ValidatorModel,
}

impl From<(u64, ValidatorModel)> for ValidatorView {
    fn from((id, model): (u64, ValidatorModel)) -> Self {
        ValidatorView {
            validator_index: id,
            model,
        }
    }
}

impl From<ValidatorView> for JsValue {
    fn from(val: ValidatorView) -> Self {
        to_js(&val).unwrap()
    }
}
