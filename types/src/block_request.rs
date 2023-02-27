use indexer_macro::Persistable;
use serde::{Deserialize, Serialize};

use crate::model::ModelWithId;

#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[persistable(prefix = "/block_requests")]
#[persistable(index = "model")]
pub struct BlockRequestModel {
    pub root: Vec<u8>,
    pub failed_count: u64,
    pub not_found_count: u64,
    pub state: String,
}
