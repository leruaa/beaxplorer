use indexer_macro::Persistable;
use indexer_macro::ToPathWithId;
use serde::Deserialize;
use serde::Serialize;

use crate::model::ModelWithId;

#[derive(Persistable, ToPathWithId, Serialize, Deserialize, Debug, Clone)]
#[persistable(index = "collection")]
#[to_path(prefix = "/blocks/c")]
pub struct CommitteeModel {
    pub index: u64,
    pub validators: Vec<u64>,
}
