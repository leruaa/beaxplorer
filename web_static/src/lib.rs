use bytes::Buf;
use futures::future::try_join_all;
use js_sys::{Array, Error};
use thiserror::Error;
use types::{meta::EpochsMeta, models::EpochModel};
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

    let epoch = rmp_serde::from_read::<_, EpochModel>(response.bytes().await?.reader())?;

    JsValue::from_serde(&epoch).map_err(Into::into)
}

#[wasm_bindgen]
pub async fn get_epochs(base: String, page_index: i32, page_size: i32) -> Result<Array, JsValue> {
    let result = get_epochs_internal(base, page_index, page_size).await;

    match result {
        Err(err) => Err(Error::new(&err.to_string()).into()),
        Ok(epoch) => Ok(epoch),
    }
}

async fn get_epochs_internal(
    base: String,
    page_index: i32,
    page_size: i32,
) -> Result<Array, DeserializeError> {
    let mut futures = vec![];
    let start_epoch = page_index * page_size + 1;
    let end_epoch = start_epoch + page_size;

    for epoch in start_epoch..end_epoch {
        futures.push(get_epoch_internal(base.clone(), epoch.to_string()));
    }

    let epochs = try_join_all(futures).await?;

    Ok(epochs.into_iter().collect::<Array>())
}

#[wasm_bindgen]
pub async fn get_epochs_meta(base: String) -> Result<JsValue, JsValue> {
    let result = get_epochs_meta_internal(base).await;

    match result {
        Err(err) => Err(Error::new(&err.to_string()).into()),
        Ok(meta) => Ok(meta),
    }
}

async fn get_epochs_meta_internal(base: String) -> Result<JsValue, DeserializeError> {
    let response = reqwest::get(format!("{}/data/epochs/meta.msg", base)).await?;

    let epoch = rmp_serde::from_read::<_, EpochsMeta>(response.bytes().await?.reader())?;

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
