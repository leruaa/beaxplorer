use crate::model::ModelWithId;
use indexer_macro::Persistable;
use serde::Deserialize;
use serde::Serialize;

#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[persistable(model = "collection")]
#[persistable(prefix = "/blocks/v")]
pub struct VoteModel {
    pub slot: u64,
    pub committee_index: u64,
    pub validators: Vec<usize>,
}
