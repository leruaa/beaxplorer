use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommiteeModel {
    pub index: u64,
    pub validators: Vec<u64>,
}

pub type CommiteesModelWithId = (u64, Vec<CommiteeModel>);
