use std::convert::TryFrom;

use serde::Serialize;
use tsify::Tsify;
use types::meta::{DepositMeta, Meta, MetaSpecific};
use wasm_bindgen::JsValue;

use crate::to_js;

#[derive(Serialize, Tsify, Debug, Clone)]
#[tsify(into_wasm_abi)]
pub struct MetaView {
    pub count: usize,
}

impl From<Meta> for MetaView {
    fn from(meta: Meta) -> Self {
        MetaView { count: meta.count }
    }
}

impl From<MetaView> for JsValue {
    fn from(val: MetaView) -> Self {
        to_js(&val).unwrap()
    }
}

#[derive(Serialize, Tsify, Debug, Clone)]
#[tsify(into_wasm_abi)]
pub struct DepositMetaView {
    pub count: usize,
    #[serde(flatten)]
    pub meta: DepositMeta,
}

impl TryFrom<Meta> for DepositMetaView {
    type Error = String;

    fn try_from(value: Meta) -> Result<Self, Self::Error> {
        match value.specific {
            MetaSpecific::Empty => Err("Invalid meta type".to_string()),
            MetaSpecific::Deposit(meta) => Ok(DepositMetaView {
                count: value.count,
                meta,
            }),
        }
    }
}

impl From<DepositMetaView> for JsValue {
    fn from(val: DepositMetaView) -> Self {
        to_js(&val).unwrap()
    }
}
