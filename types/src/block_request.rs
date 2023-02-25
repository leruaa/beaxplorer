use serde::{Deserialize, Serialize};

use crate::model::ModelWithId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockRequestModel {
    pub root: String,
    pub failed_count: u64,
    pub not_found_count: u64,
    pub state: String,
}

pub type BlockRequestModelWithId = ModelWithId<BlockRequestModel>;
