use serde::Serialize;
use tsify::Tsify;
use types::committee::CommitteeModel;

#[derive(Serialize, Tsify, Debug, Clone)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct CommitteeView {
    pub index: u64,
    pub validators: Vec<usize>,
}

impl From<CommitteeModel> for CommitteeView {
    fn from(model: CommitteeModel) -> Self {
        CommitteeView {
            index: model.index,
            validators: model.validators,
        }
    }
}
