use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone)]
pub struct EpochsMeta {
    pub count: usize,
}

impl EpochsMeta {
    pub fn new(count: usize) -> Self {
        EpochsMeta { count }
    }
}
