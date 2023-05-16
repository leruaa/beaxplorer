use serde::{Deserialize, Serialize};

use crate::{
    path::Prefix,
    persistable::{MsgPackDeserializable, MsgPackSerializable},
};

#[cfg(feature = "wasm")]
use tsify::Tsify;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
pub struct Meta {
    pub count: usize,
    pub specific: MetaSpecific,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DepositMeta {
    latest_block: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub enum MetaSpecific {
    #[default]
    Empty,
    Deposit(DepositMeta),
}

impl Meta {
    pub fn to_path<M: Prefix>(base_path: &str) -> String {
        format!("{}{}/meta.msg", base_path, M::prefix())
    }
}

impl MsgPackSerializable for Meta {}

impl MsgPackDeserializable for Meta {}
