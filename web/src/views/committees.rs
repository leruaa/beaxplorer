use serde::Serialize;
use types::committee::CommitteeModel;

#[derive(Serialize, Debug, Clone)]
pub struct CommitteeView {
    pub index: u64,
    pub validators: Vec<u64>,
}

impl From<CommitteeModel> for CommitteeView {
    fn from(model: CommitteeModel) -> Self {
        CommitteeView {
            index: model.index,
            validators: model.validators,
        }
    }
}
