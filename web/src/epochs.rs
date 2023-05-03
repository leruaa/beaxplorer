use crate::app::App;
use crate::page::{get_paths, RangeInput};
use crate::views::epochs::{EpochExtendedView, EpochPaths, EpochView};
use crate::{deserialize, PathArray};

use js_sys::{ArrayBuffer, JsString};
use types::epoch::{EpochExtendedModel, EpochModel};
use types::meta::Meta;
use types::path::ToPath;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "getEpoch")]
pub fn get_epoch(buffer: ArrayBuffer, epoch: u64) -> Result<EpochView, JsValue> {
    let model = deserialize::<EpochModel>(buffer)?;
    Ok(EpochView::from((epoch, model)))
}

#[wasm_bindgen(js_name = "getEpochPaths")]
pub fn get_epoch_paths(app: &App, epoch: u64) -> EpochPaths {
    EpochPaths {
        epoch: EpochModel::to_path(&app.base_url(), &epoch),
        epoch_extended: EpochExtendedModel::to_path(&app.base_url(), &epoch),
    }
}

#[wasm_bindgen(js_name = "getEpochRangePaths")]
pub async fn get_epoch_range_paths(
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
pub fn get_epoch_meta_path(app: &App) -> JsString {
    Meta::to_path::<EpochModel>(&app.base_url()).into()
}
