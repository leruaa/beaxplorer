use serde::Serialize;
use types::{block::BlockModel, epoch::EpochModel, validator::ValidatorModel};

use super::meta::Meta;

pub trait Model: Serialize + Send {
    fn get_id(&self) -> String;
    fn get_path() -> &'static str;
}

impl Model for EpochModel {
    fn get_id(&self) -> String {
        self.epoch.to_string()
    }

    fn get_path() -> &'static str {
        "epochs"
    }
}

impl Model for BlockModel {
    fn get_id(&self) -> String {
        self.slot.to_string()
    }

    fn get_path() -> &'static str {
        "blocks"
    }
}

impl Model for ValidatorModel {
    fn get_id(&self) -> String {
        self.validator_index.to_string()
    }

    fn get_path() -> &'static str {
        "validators"
    }
}

impl<M: Meta + Serialize> Model for M {
    fn get_id(&self) -> String {
        "meta".to_string()
    }

    fn get_path() -> &'static str {
        M::get_path()
    }
}
