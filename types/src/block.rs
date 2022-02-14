use crate::model::ModelWithId;
use serde::Deserialize;
use serde::Serialize;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Serialize, Deserialize, Debug, Clone)]
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

pub type BlockModelWithId = ModelWithId<BlockModel>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockExtendedModel {
    pub block_root: Vec<u8>,
    pub parent_root: Vec<u8>,
    pub state_root: Vec<u8>,
    pub signature: Vec<u8>,
    pub randao_reveal: Vec<u8>,
    pub graffiti: Vec<u8>,
    pub graffiti_text: String,
    pub eth1data_deposit_root: Vec<u8>,
    pub eth1data_deposit_count: u64,
    pub eth1data_block_hash: Vec<u8>,
}

pub type BlockExtendedModelWithId = ModelWithId<BlockExtendedModel>;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlocksMeta {
    pub count: usize,
}

impl BlocksMeta {
    pub fn new(count: usize) -> Self {
        BlocksMeta { count }
    }
}
