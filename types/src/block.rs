use indexer_macro::Persistable;
use serde::Deserialize;
use serde::Serialize;
#[cfg(feature = "wasm")]
use tsify::Tsify;

#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[persistable(model = "default")]
#[persistable(prefix = "/blocks")]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(rename_all = "camelCase")]
pub struct BlockModel {
    pub epoch: u64,
    pub timestamp: u64,
    pub proposer_slashings_count: usize,
    pub attester_slashings_count: usize,
    #[persistable(sortable)]
    pub attestations_count: usize,
    #[persistable(sortable)]
    pub deposits_count: usize,
    #[persistable(sortable)]
    pub voluntary_exits_count: usize,
    pub proposer: u64,
    pub status: String,
}

#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[persistable(model = "option")]
#[persistable(prefix = "/blocks/e")]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(rename_all = "camelCase")]
pub struct BlockExtendedModel {
    pub block_root: String,
    pub parent_root: String,
    pub state_root: String,
    pub signature: String,
    pub randao_reveal: String,
    pub graffiti: String,
    pub votes_count: usize,
    pub eth1data_deposit_root: String,
    pub eth1data_deposit_count: u64,
    pub eth1data_block_hash: String,
}
