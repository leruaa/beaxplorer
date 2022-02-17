use serde::Serialize;
use types::attestation::AttestationModel;

#[derive(Serialize, Debug, Clone)]
pub struct AttestationView {
    pub slot: u64,
    pub aggregation_bits: Vec<bool>,
    pub committee_index: u64,
    pub beacon_block_root: String,
    pub source: u64,
    pub target: u64,
    pub signature: String,
}

impl From<AttestationModel> for AttestationView {
    fn from(model: AttestationModel) -> Self {
        AttestationView {
            slot: model.slot,
            aggregation_bits: model.aggregation_bits,
            committee_index: model.committee_index,
            beacon_block_root: model.beacon_block_root.to_string(),
            source: model.source,
            target: model.target,
            signature: model.signature,
        }
    }
}
