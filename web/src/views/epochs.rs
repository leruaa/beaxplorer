use std::ops::Div;

use serde::Serialize;
use tsify::Tsify;
use types::epoch::{EpochExtendedModel, EpochModel};
use wasm_bindgen::JsValue;

use crate::to_js;

#[derive(Serialize, Tsify, Debug, Clone)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct EpochView {
    pub epoch: u64,
    #[serde(flatten)]
    pub model: EpochModel,
    pub finalized: bool,
    pub global_participation_rate: f64,
}

impl From<(u64, EpochModel)> for EpochView {
    fn from((epoch, model): (u64, EpochModel)) -> Self {
        let global_participation_rate = (model.voted_ether as f64).div(model.eligible_ether as f64);

        EpochView {
            epoch,
            model,
            finalized: global_participation_rate >= 2f64 / 3f64,
            global_participation_rate,
        }
    }
}

#[derive(Serialize, Tsify, Debug, Clone)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
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

impl From<EpochView> for JsValue {
    fn from(val: EpochView) -> Self {
        to_js(&val).unwrap()
    }
}

impl From<EpochExtendedView> for JsValue {
    fn from(val: EpochExtendedView) -> Self {
        to_js(&val).unwrap()
    }
}
