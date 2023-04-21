use types::deposit::{DepositModel, DepositModelWithId};
use types::path::ToPath;
use wasm_bindgen::prelude::*;

use crate::app::App;
use crate::fetcher::{fetch, fetch_meta};
use crate::views::deposits::DepositView;
use crate::views::meta::MetaView;

#[wasm_bindgen(js_name = "getDeposit")]
pub async fn get_deposit(app: &App, id: u64) -> Result<DepositView, JsValue> {
    let deposit_url = DepositModelWithId::to_path(&app.base_url(), &id);
    let deposit = fetch::<DepositModel>(deposit_url).await?;
 
    Ok(DepositView::from(deposit))
}

#[wasm_bindgen(js_name = "getDepositMeta")]
pub async fn get_deposit_meta(app: &App) -> Result<MetaView, JsValue> {
    fetch_meta::<DepositModel>(app).await
}
