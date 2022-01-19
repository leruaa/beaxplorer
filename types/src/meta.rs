use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EpochsMeta {
    pub count: usize,
}

impl EpochsMeta {
    pub fn new(count: usize) -> Self {
        EpochsMeta { count }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlocksMeta {
    pub count: usize,
}

impl BlocksMeta {
    pub fn new(count: usize) -> Self {
        BlocksMeta { count }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValidatorsMeta {
    pub count: usize,
}

impl ValidatorsMeta {
    pub fn new(count: usize) -> Self {
        ValidatorsMeta { count }
    }
}
