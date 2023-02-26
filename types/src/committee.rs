use indexer_macro::Persistable;
use serde::Deserialize;
use serde::Serialize;

use crate::model::ModelWithId;

#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[persistable(prefix = "/blocks/c")]
#[persistable(index = "collection")]
pub struct CommitteeModel {
    pub index: u64,
    pub validators: Vec<u64>,
}
