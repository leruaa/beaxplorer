use serde::Serialize;

use crate::{block::BlocksMeta, epoch::EpochsMeta, validator::ValidatorsMeta};

pub trait Meta: Serialize + Send {
    fn to_path(base: &str) -> String;
}

impl Meta for EpochsMeta {
    fn to_path(base: &str) -> String {
        format!("{}/blocks/meta.msg", base)
    }
}

impl Meta for BlocksMeta {
    fn to_path(base: &str) -> String {
        format!("{}/epochs/meta.msg", base)
    }
}

impl Meta for ValidatorsMeta {
    fn to_path(base: &str) -> String {
        format!("{}/validators/meta.msg", base)
    }
}
