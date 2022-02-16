use lighthouse_types::Hash256;
use serde::Deserialize;
use serde::Serialize;

use crate::model::ModelWithId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttestationModel {
    pub slot: u64,
    pub aggregation_bits: Vec<bool>,
    pub committee_index: u64,
    pub beacon_block_root: Hash256,
    pub source: u64,
    pub target: u64,
    pub signature: String,
}

pub type AttestationsModelWithId = ModelWithId<Vec<AttestationModel>>;

#[cfg(feature = "indexing")]
impl<T: lighthouse_types::EthSpec> From<&lighthouse_types::Attestation<T>> for AttestationModel {
    fn from(attestation: &lighthouse_types::Attestation<T>) -> Self {
        AttestationModel {
            slot: attestation.data.slot.as_u64(),
            aggregation_bits: attestation.aggregation_bits.iter().collect(),
            committee_index: attestation.data.index,
            beacon_block_root: attestation.data.beacon_block_root,
            source: attestation.data.source.epoch.as_u64(),
            target: attestation.data.target.epoch.as_u64(),
            signature: attestation.signature.to_string(),
        }
    }
}
