use serde::Serialize;
use tsify::Tsify;
use types::block::{BlockExtendedModel, BlockModel, BlockModelWithId};
use wasm_bindgen::JsValue;

use crate::to_js;

#[derive(Serialize, Debug, Clone)]
pub struct BlockView {
    pub slot: u64,
    #[serde(flatten)]
    pub model: BlockModel,
}

impl From<BlockModelWithId> for BlockView {
    fn from(value: BlockModelWithId) -> Self {
        BlockView {
            slot: value.id,
            model: value.model,
        }
    }
}

#[derive(Serialize, Tsify, Debug, Clone)]
#[tsify(into_wasm_abi)]
pub struct BlockExtendedView {
    pub slot: u64,
    #[serde(flatten)]
    pub model: BlockModel,
    #[serde(flatten)]
    pub extended_model: BlockExtendedModel,
}

impl From<(u64, BlockModel, BlockExtendedModel)> for BlockExtendedView {
    fn from((slot, model, extended_model): (u64, BlockModel, BlockExtendedModel)) -> Self {
        BlockExtendedView {
            slot,
            model,
            extended_model,
        }
    }
}

impl From<BlockExtendedView> for JsValue {
    fn from(val: BlockExtendedView) -> Self {
        to_js(&val).unwrap()
    }
}
