use indexer_macro::{Persistable, ToPathWithId};
use serde::{Deserialize, Serialize};

use crate::model::ModelWithId;

#[derive(Persistable, ToPathWithId, Serialize, Deserialize, Debug, Clone)]
#[persistable(index = "model")]
#[to_path(prefix = "/block_requests")]
pub struct BlockRequestModel {
    pub root: Vec<u8>,
    pub failed_count: u64,
    pub not_found_count: u64,
    pub state: String,
}
