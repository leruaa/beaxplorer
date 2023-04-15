use serde::Serialize;
use tsify::Tsify;
use types::attestation::AttestationModel;

#[derive(Serialize, Tsify, Debug, Clone)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct AttestationView {
    #[serde(flatten)]
    pub model: AttestationModel,
}

impl From<AttestationModel> for AttestationView {
    fn from(model: AttestationModel) -> Self {
        AttestationView { model }
    }
}
