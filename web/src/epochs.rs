use crate::app::App;
use crate::fetcher::{fetch, fetch_meta};
use crate::views::epochs::{EpochExtendedView, EpochView};
use crate::views::meta::MetaView;

use types::epoch::{EpochExtendedModel, EpochExtendedModelWithId, EpochModel, EpochModelWithId};
use types::path::ToPath;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "getEpoch")]
pub async fn get_epoch(app: &App, epoch: u64) -> Result<EpochView, JsValue> {
    let epoch_url = EpochModelWithId::to_path(&app.base_url(), &epoch);

    let model = fetch::<EpochModel>(epoch_url).await?;
    Ok(EpochView::from((epoch, model)))
}

#[wasm_bindgen(js_name = "getEpochExtended")]
pub async fn get_epoch_extended(app: &App, epoch: u64) -> Result<EpochExtendedView, JsValue> {
    let epoch_url = EpochModelWithId::to_path(&app.base_url(), &epoch);
    let extended_epoch_url = EpochExtendedModelWithId::to_path(&app.base_url(), &epoch);

    let model = fetch::<EpochModel>(epoch_url).await?;
    let extended_model = fetch::<EpochExtendedModel>(extended_epoch_url).await?;
    Ok(EpochExtendedView::from((epoch, model, extended_model)))
}

#[wasm_bindgen(js_name = "getEpochMeta")]
pub async fn get_epoch_meta(app: &App) -> Result<MetaView, JsValue> {
    fetch_meta::<EpochModel>(app).await
}
