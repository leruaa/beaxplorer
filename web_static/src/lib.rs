use js_sys::Error;
use thiserror::Error;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub mod blocks;
pub mod epochs;
mod fetcher;
mod get;
mod page;
pub mod sort;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[derive(Error, Debug)]
pub enum DeserializeError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeRmp(#[from] rmp_serde::decode::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_wasm_bindgen::Error),
}

impl From<DeserializeError> for JsValue {
    fn from(err: DeserializeError) -> Self {
        Error::new(&err.to_string()).into()
    }
}

pub fn to_js<T: types::Serialize + ?Sized>(value: &T) -> Result<JsValue, DeserializeError> {
    serde_wasm_bindgen::to_value(value).map_err(Into::into)
}
