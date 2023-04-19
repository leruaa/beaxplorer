use crate::{app::App, views::meta::MetaView, DeserializeError};
use bytes::Buf;
use types::{meta::Meta, path::Prefix, DeserializeOwned};
use wasm_bindgen::JsValue;

pub async fn fetch<T: DeserializeOwned>(url: String) -> Result<T, DeserializeError> {
    let response = reqwest::get(url).await?;

    rmp_serde::from_read::<_, T>(response.bytes().await?.reader()).map_err(Into::into)
}

pub async fn fetch_meta<M: Prefix>(app: &App) -> Result<MetaView, JsValue> {
    let meta_url = Meta::to_path::<M>(&app.base_url());

    fetch::<Meta>(meta_url)
        .await
        .map(Into::into)
        .map_err(Into::into)
}
