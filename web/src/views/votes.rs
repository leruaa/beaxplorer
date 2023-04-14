use serde::Serialize;
use tsify::Tsify;
use types::vote::VoteModel;

#[derive(Serialize, Tsify, Debug, Clone)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
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
