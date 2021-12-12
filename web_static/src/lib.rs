use bytes::Buf;
use js_sys::Error;
use thiserror::Error;
use types::models::EpochModel;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn get_epoch(base: String, epoch: String) -> Result<JsValue, JsValue> {
    let result = get_epoch_internal(base, epoch).await;

    match result {
        Err(err) => Err(Error::new(&err.to_string()).into()),
        Ok(epoch) => Ok(epoch),
    }
}

async fn get_epoch_internal(base: String, epoch: String) -> Result<JsValue, DeserializeError> {
    let response = reqwest::get(format!("{}/data/epochs/{}.msg", base, epoch)).await?;

    let epoch = rmp_serde::from_read::<_, EpochModel>(response.bytes().await.unwrap().reader())?;

    JsValue::from_serde(&epoch).map_err(Into::into)
}

#[derive(Error, Debug)]
pub enum DeserializeError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeRmp(#[from] rmp_serde::decode::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::error::Error),
}
