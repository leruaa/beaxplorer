use std::ops::Div;

use crate::meta::Meta;
use crate::meta::WithMeta;
use crate::model::ModelWithId;
use crate::utils::Orderable;
use indexer_macro::Persistable;
use indexer_macro::ToPath;
use indexer_macro::ToPathWithId;
use ordered_float::OrderedFloat;
use serde::Deserialize;
use serde::Serialize;
#[cfg(feature = "wasm")]
use tsify::Tsify;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Persistable, ToPathWithId, Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[persistable(index = "model")]
#[persistable(sortable_field(
    name = "global_participation_rate",
    ty = "OrderedFloat<f64>",
    with = "get_global_participation_rate"
))]
#[to_path(prefix = "/epochs")]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
pub struct EpochModel {
    pub timestamp: u64,
    #[persistable(sortable)]
    pub proposer_slashings_count: usize,
    #[persistable(sortable)]
    pub attester_slashings_count: usize,
    #[persistable(sortable)]
    pub attestations_count: usize,
    #[persistable(sortable)]
    pub deposits_count: usize,
    #[persistable(sortable)]
    pub eligible_ether: u64,
    #[persistable(sortable)]
    pub voted_ether: u64,
}

impl WithMeta for EpochModel {
    type MetaType = EpochsMeta;
}

// global_participation_rate
fn get_global_participation_rate(value: &EpochModelWithId) -> Orderable<OrderedFloat<f64>> {
    let global_participation_rate =
        (value.model.voted_ether as f64).div(value.model.eligible_ether as f64);
    (value.id, OrderedFloat(global_participation_rate)).into()
}

#[derive(Persistable, ToPathWithId, Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[persistable(index = "model")]
#[to_path(prefix = "/epochs/e")]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
pub struct EpochExtendedModel {
    pub voluntary_exits_count: usize,
    pub validators_count: usize,
    pub average_validator_balance: u64,
    pub total_validator_balance: u64,
}

#[derive(Serialize, Debug, Clone)]
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

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(ToPath, Serialize, Deserialize, Debug, Clone)]
#[to_path(prefix = "/epochs/meta")]
pub struct EpochsMeta {
    pub count: usize,
}

impl EpochsMeta {
    pub fn new(count: usize) -> Self {
        EpochsMeta { count }
    }
}

impl Meta for EpochsMeta {
    fn count(&self) -> usize {
        self.count
    }
}
