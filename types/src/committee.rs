use serde::Deserialize;
use serde::Serialize;

use crate::model::ModelWithId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommitteeModel {
    pub index: u64,
    pub validators: Vec<u64>,
}

pub type CommitteesModelWithId = ModelWithId<Vec<CommitteeModel>>;
