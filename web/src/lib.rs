use js_sys::{ArrayBuffer, Error, Uint8Array};
use page::ModelId;
use serde::Serialize;
use thiserror::Error;
use types::{meta::Meta, DeserializeOwned};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::views::meta::MetaView;

pub mod app;
pub mod block_requests;
pub mod blocks;
pub mod deposits;
pub mod epochs;
mod fetcher;
pub mod good_peers;
mod page;
pub mod sort;
pub mod validators;
pub mod views;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "string[]")]
    pub type StringArray;

    #[wasm_bindgen(typescript_type = "[bigint | string, string][]")]
    pub type PathArray;

    #[wasm_bindgen(typescript_type = "CommitteeView[]")]
    pub type CommitteeArray;

    #[wasm_bindgen(typescript_type = "VoteView[]")]
    pub type VoteArray;

    #[wasm_bindgen(typescript_type = "AttestationView[]")]
    pub type AttestationArray;

    #[wasm_bindgen(typescript_type = "ValidatorView[]")]
    pub type ValidatorArray;
}

#[wasm_bindgen(js_name = "getMeta")]
pub fn get_meta(meta_buffer: ArrayBuffer) -> Result<MetaView, JsValue> {
    deserialize::<Meta>(meta_buffer)
        .map(Into::into)
        .map_err(Into::into)
}

#[derive(Error, Debug)]
pub enum DeserializeError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeRmp(#[from] rmp_serde::decode::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_wasm_bindgen::Error),

    #[error("Invalid model id {0}")]
    InvalidModelId(ModelId),
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

pub fn to_js_with_large_numbers_as_bigints<T: Serialize + ?Sized>(
    value: &T,
) -> Result<JsValue, DeserializeError> {
    value
        .serialize(
            &serde_wasm_bindgen::Serializer::json_compatible()
                .serialize_large_number_types_as_bigints(true),
        )
        .map_err(Into::into)
}

pub fn deserialize<T>(buffer: ArrayBuffer) -> Result<T, DeserializeError>
where
    T: DeserializeOwned,
{
    let array = Uint8Array::new(&buffer);

    let mut buf = vec![0_u8; array.length() as usize];

    array.copy_to(buf.as_mut());

    rmp_serde::from_slice::<T>(buf.as_ref()).map_err(Into::into)
}
