use serde::Serialize;
use tsify::Tsify;
use types::block_request::BlockRequestModel;
use wasm_bindgen::JsValue;

use crate::to_js;

#[derive(Serialize, Tsify, Debug, Clone)]
#[tsify(into_wasm_abi)]
pub struct BlockRequestView {
    #[serde(flatten)]
    pub model: BlockRequestModel,
}

impl From<BlockRequestModel> for BlockRequestView {
    fn from(model: BlockRequestModel) -> Self {
        BlockRequestView { model }
    }
}

impl From<BlockRequestView> for JsValue {
    fn from(val: BlockRequestView) -> Self {
        to_js(&val).unwrap()
    }
}
