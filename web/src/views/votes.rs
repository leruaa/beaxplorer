use serde::Serialize;
use types::vote::VoteModel;

#[derive(Serialize, Debug, Clone)]
pub struct VoteView {
    pub slot: u64,
    pub committee_index: u64,
    pub validators: Vec<usize>,
}

impl From<VoteModel> for VoteView {
    fn from(model: VoteModel) -> Self {
        VoteView {
            slot: model.slot,
            committee_index: model.committee_index,
            validators: model.validators,
        }
    }
}
