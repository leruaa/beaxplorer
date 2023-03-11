use js_sys::Error;
use serde::Serialize;
use thiserror::Error;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub mod app;
pub mod block_requests;
pub mod blocks;
pub mod epochs;
mod fetcher;
mod page;
pub mod sort;
pub mod validators;
pub mod views;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "CommitteeView[]")]
    pub type CommitteeArray;

    #[wasm_bindgen(typescript_type = "VoteView[]")]
    pub type VoteArray;

    #[wasm_bindgen(typescript_type = "AttestationView[]")]
    pub type AttestationArray;

    #[wasm_bindgen(typescript_type = "ValidatorView[]")]
    pub type ValidatorArray;

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

pub fn to_js<T: Serialize + ?Sized>(value: &T) -> Result<JsValue, DeserializeError> {
    value
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(Into::into)
}
