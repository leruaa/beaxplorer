use js_sys::{ArrayBuffer, JsString};
use types::meta::Meta;
use types::path::ToPath;
use types::validator::{ValidatorExtendedModel, ValidatorModel};
use wasm_bindgen::prelude::*;

use crate::app::App;
use crate::page::{get_paths, RangeInput};
use crate::views::validators::{ValidatorPaths, ValidatorView};
use crate::{deserialize, PathArray};

#[wasm_bindgen(js_name = "getValidator")]
pub fn get_validator(buffer: ArrayBuffer, index: u64) -> Result<ValidatorView, JsValue> {
    let model = deserialize::<ValidatorModel>(buffer)?;
    Ok(ValidatorView::from((index, model)))
}

#[wasm_bindgen(js_name = "getValidatorPaths")]
pub fn get_validator_paths(app: &App, index: u64) -> ValidatorPaths {
    ValidatorPaths {
        validator: ValidatorModel::to_path(&app.base_url(), &index),
        validator_extended: ValidatorExtendedModel::to_path(&app.base_url(), &index),
    }
}

#[wasm_bindgen(js_name = "getValidatorRangePaths")]
pub async fn get_epoch_range_paths(
    app: &App,
    input: RangeInput,
    total_count: usize,
) -> Result<PathArray, JsValue> {
    get_paths::<ValidatorModel>(app, input, total_count).await
}

#[wasm_bindgen(js_name = "getValidatorMetaPath")]
pub fn get_epoch_meta_path(app: &App) -> JsString {
    Meta::to_path::<ValidatorModel>(&app.base_url()).into()
}
