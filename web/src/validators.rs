use types::path::ToPath;
use types::validator::{ValidatorModel, ValidatorModelWithId};
use wasm_bindgen::prelude::*;

use crate::app::App;
use crate::fetcher::{fetch, fetch_meta};
use crate::views::meta::MetaView;
use crate::views::validators::ValidatorView;

#[wasm_bindgen(js_name = "getValidator")]
pub async fn get_validator(app: &App, id: u64) -> Result<ValidatorView, JsValue> {
    let validator_url = ValidatorModelWithId::to_path(&app.base_url(), &id);
    let validator = fetch::<ValidatorModel>(validator_url).await?;

    Ok(ValidatorView::from((id, validator)))
}

#[wasm_bindgen(js_name = "getValidatorMeta")]
pub async fn get_calidator_meta(app: &App) -> Result<MetaView, JsValue> {
    fetch_meta::<ValidatorModel>(app).await
}
