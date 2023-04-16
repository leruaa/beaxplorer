use serde::Serialize;
use tsify::Tsify;
use types::vote::VoteModel;

#[derive(Serialize, Tsify, Debug, Clone)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct VoteView {
    #[serde(flatten)]
    pub model: VoteModel,
}

impl From<VoteModel> for VoteView {
    fn from(model: VoteModel) -> Self {
        VoteView { model }
    }
}
