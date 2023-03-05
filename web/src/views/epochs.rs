use std::ops::Div;

use serde::Serialize;
use tsify::Tsify;
use types::epoch::{EpochExtendedModel, EpochModel, EpochModelWithId};
use wasm_bindgen::JsValue;

use crate::to_js;

#[derive(Serialize, Tsify, Debug, Clone)]
#[tsify(into_wasm_abi)]
pub struct EpochView {
    pub epoch: u64,
    #[serde(flatten)]
    pub model: EpochModel,
    pub finalized: bool,
    pub global_participation_rate: f64,
}

impl From<EpochModelWithId> for EpochView {
    fn from(value: EpochModelWithId) -> Self {
        let global_participation_rate =
            (value.model.voted_ether as f64).div(value.model.eligible_ether as f64);

        EpochView {
            epoch: value.id,
            model: value.model,
            finalized: global_participation_rate >= 2f64 / 3f64,
            global_participation_rate,
        }
    }
}

#[derive(Serialize, Tsify, Debug, Clone)]
#[tsify(into_wasm_abi)]
pub struct EpochExtendedView {
    pub epoch: u64,
    #[serde(flatten)]
    pub model: EpochModel,
    pub finalized: bool,
    pub global_participation_rate: f64,
    #[serde(flatten)]
    pub extended_model: EpochExtendedModel,
}

impl From<(u64, EpochModel, EpochExtendedModel)> for EpochExtendedView {
    fn from((epoch, model, extended_model): (u64, EpochModel, EpochExtendedModel)) -> Self {
        let global_participation_rate = (model.voted_ether as f64).div(model.eligible_ether as f64);

        EpochExtendedView {
            epoch,
            model,
            finalized: global_participation_rate >= 2f64 / 3f64,
            global_participation_rate,
            extended_model,
        }
    }
}

impl From<EpochExtendedView> for JsValue {
    fn from(val: EpochExtendedView) -> Self {
        to_js(&val).unwrap()
    }
}
