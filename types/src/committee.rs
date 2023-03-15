use indexer_macro::Persistable;
use serde::Deserialize;
use serde::Serialize;

use crate::model::ModelWithId;

#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[persistable(model = "collection")]
#[persistable(prefix = "/blocks/c")]
pub struct CommitteeModel {
    pub index: u64,
    pub validators: Vec<u64>,
}
