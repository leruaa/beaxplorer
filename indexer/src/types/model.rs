use serde::Serialize;
use types::{
    block::{BlockModel, BlocksMeta},
    epoch::{EpochExtendedModel, EpochModel, EpochsMeta},
    validator::{ValidatorModel, ValidatorsMeta},
};

pub trait Model: Serialize + Send {
    fn get_path() -> &'static str;
}

impl Model for EpochModel {
    fn get_path() -> &'static str {
        "epochs"
    }
}
