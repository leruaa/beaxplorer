use thiserror::Error;
use wasm_bindgen::prelude::wasm_bindgen;

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
    SerdeJson(#[from] serde_json::error::Error),
}
