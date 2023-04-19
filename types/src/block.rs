use indexer_macro::Persistable;
use serde::Deserialize;
use serde::Serialize;
#[cfg(feature = "wasm")]
use tsify::Tsify;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[persistable(model = "default")]
#[persistable(prefix = "/blocks")]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
pub struct BlockModel {
    pub epoch: u64,
    pub proposer_slashings_count: usize,
    pub attester_slashings_count: usize,
    pub attestations_count: usize,
    pub deposits_count: usize,
    pub voluntary_exits_count: usize,
    pub proposer: u64,
    pub status: String,
}

#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[persistable(model = "option")]
#[persistable(prefix = "/blocks/e")]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
pub struct BlockExtendedModel {
    pub block_root: Vec<u8>,
    pub parent_root: Vec<u8>,
    pub state_root: Vec<u8>,
    pub signature: Vec<u8>,
    pub randao_reveal: Vec<u8>,
    pub graffiti: Vec<u8>,
    pub graffiti_text: String,
    pub votes_count: usize,
    pub eth1data_deposit_root: Vec<u8>,
    pub eth1data_deposit_count: u64,
    pub eth1data_block_hash: Vec<u8>,
}
