use std::convert::TryFrom;

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

impl Meta {
    pub fn deposit_default() -> Self {
        Self {
            count: 0,
            specific: MetaSpecific::Deposit(DepositMeta::default()),
        }
    }

    pub fn to_path<M: Prefix>(base_path: &str) -> String {
        format!("{}{}/meta.msg", base_path, M::prefix())
    }

    pub fn save<M: Prefix>(&self, base_path: &str) -> Result<(), String> {
        self.serialize_to_file(&Meta::to_path::<M>(base_path))
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DepositMeta {
    pub latest_block: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub enum MetaSpecific {
    #[default]
    Empty,
    Deposit(DepositMeta),
}

impl<'a> TryFrom<&'a Meta> for &'a DepositMeta {
    type Error = String;

    fn try_from(value: &'a Meta) -> Result<Self, Self::Error> {
        match &value.specific {
            MetaSpecific::Empty => Err("Invalid meta type".to_string()),
            MetaSpecific::Deposit(d) => Ok(d),
        }
    }
}

impl<'a> TryFrom<&'a mut Meta> for &'a mut DepositMeta {
    type Error = String;

    fn try_from(value: &'a mut Meta) -> Result<Self, Self::Error> {
        match &mut value.specific {
            MetaSpecific::Empty => Err("Invalid meta type".to_string()),
            MetaSpecific::Deposit(d) => Ok(d),
        }
    }
}

impl MsgPackSerializable for Meta {}

impl MsgPackDeserializable for Meta {}
