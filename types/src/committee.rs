use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommitteeModel {
    pub index: u64,
    pub validators: Vec<u64>,
}

pub type CommitteesModelWithId = (u64, Vec<CommitteeModel>);
