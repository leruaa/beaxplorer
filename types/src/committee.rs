use indexer_macro::Persistable;
use serde::Deserialize;
use serde::Serialize;
#[cfg(feature = "wasm")]
use tsify::Tsify;

#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[persistable(model = "collection")]
#[persistable(prefix = "/blocks/c")]
#[serde(rename_all = "camelCase")]
pub struct CommitteeModel {
    pub index: u64,
    pub validators: Vec<usize>,
}

#[cfg(feature = "indexing")]
impl From<&lighthouse_types::OwnedBeaconCommittee> for CommitteeModel {
    fn from(value: &lighthouse_types::OwnedBeaconCommittee) -> Self {
        CommitteeModel {
            index: value.index,
            validators: value.committee.to_vec(),
        }
    }
}
