use bytes::Buf;
use js_sys::{Error, Promise};
use types::{meta::EpochsMeta, views::EpochView};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::{get::get, page::page, DeserializeError};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

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
        let url = base_url + "/data/epochs";
        let meta = Epochs::get_epochs_meta(url.clone())
            .await
            .map_err(|err| Error::new(&err.to_string()))?;

        Ok(Epochs::new(url, meta).into())
    }

    pub fn get(&self, epoch: String) -> Promise {
        get::<EpochView>(self.base_url.clone(), epoch)
    }

    pub fn page(
        &self,
        page_index: usize,
        page_size: usize,
        sort_id: String,
        sort_desc: bool,
    ) -> Promise {
        page::<EpochView>(
            self.base_url.clone(),
            page_index,
            page_size,
            sort_id,
            sort_desc,
            self.meta.count.clone(),
        )
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
        let response = reqwest::get(format!("{}/meta.msg", base_url)).await?;

        rmp_serde::from_read::<_, EpochsMeta>(response.bytes().await?.reader()).map_err(Into::into)
    }
}
