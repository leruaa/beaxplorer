use crate::app::App;
use crate::page::{get_paths, RangeInput};
use crate::views::epochs::{EpochExtendedView, EpochView};
use crate::{deserialize, PathArray};

use js_sys::ArrayBuffer;
use types::epoch::{EpochExtendedModel, EpochModel};
use types::meta::Meta;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "getEpoch")]
pub fn get_epoch(buffer: ArrayBuffer, epoch: u64) -> Result<EpochView, JsValue> {
    let model = deserialize::<EpochModel>(buffer)?;
    Ok(EpochView::from((epoch, model)))
}

#[wasm_bindgen(js_name = "getEpochPaths")]
pub async fn get_epoch_paths(
    app: &App,
    input: RangeInput,
    total_count: usize,
) -> Result<PathArray, JsValue> {
    get_paths::<EpochModel>(app, input, total_count).await
}

#[wasm_bindgen(js_name = "getEpochExtended")]
pub fn get_epoch_extended(
    model_buffer: ArrayBuffer,
    extended_model_buffer: ArrayBuffer,
    epoch: u64,
) -> Result<EpochExtendedView, JsValue> {
    let model = deserialize::<EpochModel>(model_buffer)?;
    let extended_model = deserialize::<EpochExtendedModel>(extended_model_buffer)?;
    Ok(EpochExtendedView::from((epoch, model, extended_model)))
}

#[wasm_bindgen(js_name = "getEpochMetaPath")]
pub fn get_epoch_meta_path(app: &App) -> JsValue {
    Meta::to_path::<EpochModel>(&app.base_url()).into()
}
