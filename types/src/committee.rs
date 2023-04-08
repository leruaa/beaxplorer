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

#[cfg(feature = "indexing")]
impl From<&lighthouse_types::OwnedBeaconCommittee> for CommitteeModel {
    fn from(value: &lighthouse_types::OwnedBeaconCommittee) -> Self {
        CommitteeModel {
            index: value.index,
            validators: value.committee.iter().map(|i| *i as u64).collect(),
        }
    }
}
