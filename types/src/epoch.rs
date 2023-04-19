use std::ops::Div;

use crate::utils::Orderable;
use indexer_macro::Persistable;
use ordered_float::OrderedFloat;
use serde::Deserialize;
use serde::Serialize;
#[cfg(feature = "wasm")]
use tsify::Tsify;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[persistable(model = "default")]
#[persistable(sortable_field(
    name = "global_participation_rate",
    ty = "OrderedFloat<f64>",
    with = "get_global_participation_rate"
))]
#[persistable(prefix = "/epochs")]
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

fn get_global_participation_rate(value: &EpochModelWithId) -> Orderable<u64, OrderedFloat<f64>> {
    let global_participation_rate =
        (value.model.voted_ether as f64).div(value.model.eligible_ether as f64);
    (value.id, OrderedFloat(global_participation_rate)).into()
}

#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[persistable(model = "default")]
#[persistable(prefix = "/epochs/e")]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
pub struct EpochExtendedModel {
    pub voluntary_exits_count: usize,
    pub validators_count: usize,
    pub average_validator_balance: u64,
    pub total_validator_balance: u64,
}
