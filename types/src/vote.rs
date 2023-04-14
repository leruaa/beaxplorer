use crate::model::ModelWithId;
use indexer_macro::Persistable;
use serde::Deserialize;
use serde::Serialize;
#[cfg(feature = "wasm")]
use tsify::Tsify;

#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[persistable(model = "collection")]
#[persistable(prefix = "/blocks/v")]
#[serde(rename_all = "camelCase")]
pub struct VoteModel {
    pub slot: u64,
    pub committee_index: u64,
    pub validators: Vec<usize>,
}
