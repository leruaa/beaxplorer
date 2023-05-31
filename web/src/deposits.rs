use js_sys::{ArrayBuffer, JsString};
use types::deposit::ExecutionLayerDepositModel;
use types::meta::Meta;
use types::path::ToPath;
use wasm_bindgen::prelude::*;

use crate::app::App;
use crate::page::{get_paths, RangeInput};
use crate::views::deposits::{DepositPaths, ExecutionLayerDepositView};

use crate::{deserialize, PathArray};

#[wasm_bindgen(js_name = "getExecutionLayerDeposit")]
pub fn get_el_deposit(
    buffer: ArrayBuffer,
    index: u64,
) -> Result<ExecutionLayerDepositView, JsValue> {
    let model = deserialize::<ExecutionLayerDepositModel>(buffer)?;
    Ok(ExecutionLayerDepositView::from((index, model)))
}

#[wasm_bindgen(js_name = "getDepositPaths")]
pub fn get_deposit_paths(app: &App, index: u64) -> DepositPaths {
    DepositPaths {
        el_deposit: ExecutionLayerDepositModel::to_path(&app.base_url(), &index),
    }
}

#[wasm_bindgen(js_name = "getExecutionLayerDepositRangePaths")]
pub async fn get_el_deposit_range_paths(
    app: &App,
    input: RangeInput,
    total_count: usize,
) -> Result<PathArray, JsValue> {
    get_paths::<ExecutionLayerDepositModel>(app, input, total_count).await
}

#[wasm_bindgen(js_name = "getExecutionLayerDepositMetaPath")]
pub fn get_el_deposit_meta_path(app: &App) -> JsString {
    Meta::to_path::<ExecutionLayerDepositModel>(&app.base_url()).into()
}
