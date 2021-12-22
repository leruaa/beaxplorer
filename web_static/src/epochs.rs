use std::cmp::min;

use bytes::Buf;
use futures::future::try_join_all;
use js_sys::{Array, Error, Promise};
use types::{meta::EpochsMeta, views::EpochView};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::DeserializeError;

#[wasm_bindgen]
pub struct Epochs {
    base_url: String,
    meta: EpochsMeta,
}

#[wasm_bindgen]
impl Epochs {
    fn new(base_url: String, meta: EpochsMeta) -> Epochs {
        Epochs { base_url, meta }
    }

    #[wasm_bindgen]
    pub async fn build(base_url: String) -> Result<Epochs, JsValue> {
        let meta = Epochs::get_epochs_meta(base_url.clone())
            .await
            .map_err(|err| Error::new(&err.to_string()))?;

        Ok(Epochs::new(base_url, meta).into())
    }

    pub fn get(&self, epoch: String) -> Promise {
        let base_url = self.base_url.clone();

        future_to_promise(async move {
            let result = Self::get_epoch(base_url, epoch).await;

            match result {
                Err(err) => Err(Error::new(&err.to_string()).into()),
                Ok(epoch) => Ok(epoch),
            }
        })
    }

    async fn get_epoch(base_url: String, epoch: String) -> Result<JsValue, DeserializeError> {
        let response = reqwest::get(format!("{}/data/epochs/{}.msg", base_url, epoch)).await?;

        let epoch = rmp_serde::from_read::<_, EpochView>(response.bytes().await?.reader())?;

        JsValue::from_serde(&epoch).map_err(Into::into)
    }

    pub fn page(&self, page_index: i32, page_size: i32) -> Promise {
        let base_url = self.base_url.clone();
        let count = self.meta.count.clone();

        future_to_promise(async move {
            let result = Self::get_paginated_epochs(base_url, page_index, page_size, count).await;

            match result {
                Err(err) => Err(Error::new(&err.to_string()).into()),
                Ok(epoch) => Ok(epoch),
            }
        })
    }

    async fn get_paginated_epochs(
        base_url: String,
        page_index: i32,
        page_size: i32,
        count: usize,
    ) -> Result<JsValue, DeserializeError> {
        let mut futures = vec![];
        let start_epoch = page_index * page_size + 1;
        let end_epoch = min(start_epoch + page_size, count as i32 - 1);

        for epoch in start_epoch..end_epoch {
            futures.push(Self::get_epoch(base_url.clone(), epoch.to_string()));
        }

        let epochs = try_join_all(futures).await?;

        Ok(epochs.into_iter().collect::<Array>().into())
    }

    pub fn meta(&self) -> Promise {
        let base_url = self.base_url.clone();

        future_to_promise(async move {
            let meta = Self::get_epochs_meta(base_url).await;

            match meta {
                Ok(meta) => {
                    let result = JsValue::from_serde(&meta);

                    match result {
                        Ok(meta) => Ok(meta),
                        Err(err) => Err(Error::new(&err.to_string()).into()),
                    }
                }
                Err(err) => Err(Error::new(&err.to_string()).into()),
            }
        })
    }

    async fn get_epochs_meta(base_url: String) -> Result<EpochsMeta, DeserializeError> {
        let response = reqwest::get(format!("{}/data/epochs/meta.msg", base_url)).await?;

        rmp_serde::from_read::<_, EpochsMeta>(response.bytes().await?.reader()).map_err(Into::into)
    }
}
