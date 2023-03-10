use crate::app::App;
use crate::fetcher::fetch;
use crate::views::epochs::EpochExtendedView;

use types::epoch::{
    EpochExtendedModel, EpochExtendedModelWithId, EpochModel, EpochModelWithId, EpochsMeta,
};
use types::path::ToPath;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "getEpoch")]
pub async fn get_epoch(app: &App, epoch: u64) -> Result<EpochExtendedView, JsValue> {
    let epoch_url = EpochModelWithId::to_path(&app.base_url(), epoch);
    let extended_epoch_url = EpochExtendedModelWithId::to_path(&app.base_url(), epoch);

    let model = fetch::<EpochModel>(epoch_url).await?;
    let extended_model = fetch::<EpochExtendedModel>(extended_epoch_url).await?;
    Ok(EpochExtendedView::from((epoch, model, extended_model)))
}

#[wasm_bindgen(js_name = "getEpochMeta")]
pub async fn get_epoch_meta(app: &App) -> Result<EpochsMeta, JsValue> {
    let meta_url = EpochsMeta::to_path(&app.base_url(), ());

    fetch::<EpochsMeta>(meta_url).await.map_err(Into::into)
}
