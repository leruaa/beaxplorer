use std::convert::TryFrom;

use js_sys::ArrayBuffer;
use types::deposit::{DepositModel, DepositModelWithId};
use types::meta::Meta;
use types::path::ToPath;
use wasm_bindgen::prelude::*;

use crate::app::App;
use crate::fetcher::fetch;
use crate::views::deposits::DepositView;
use crate::views::meta::DepositMetaView;
use crate::{deserialize, DeserializeError};

#[wasm_bindgen(js_name = "getDeposit")]
pub async fn get_deposit(app: &App, id: u64) -> Result<DepositView, JsValue> {
    let deposit_url = DepositModelWithId::to_path(&app.base_url(), &id);
    let deposit = fetch::<DepositModel>(deposit_url).await?;

    Ok(DepositView::from(deposit))
}

#[wasm_bindgen(js_name = "getDepositMeta")]
pub fn get_deposit_meta(meta_buffer: ArrayBuffer) -> Result<DepositMetaView, JsValue> {
    deserialize::<Meta>(meta_buffer)
        .and_then(|m| DepositMetaView::try_from(m).map_err(|_| DeserializeError::InvalidMetaType))
        .map_err(Into::into)
}
