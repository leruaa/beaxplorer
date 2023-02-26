use indexer_macro::Persistable;
use serde::Deserialize;
use serde::Serialize;

use crate::model::ModelWithId;

#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[persistable(prefix = "/blocks/a")]
#[persistable(index = "collection")]
pub struct AttestationModel {
    pub slot: u64,
    pub aggregation_bits: Vec<bool>,
    pub committee_index: u64,
    pub beacon_block_root: String,
    pub source: u64,
    pub target: u64,
    pub signature: String,
}

#[cfg(feature = "indexing")]
impl<T: lighthouse_types::EthSpec> From<&lighthouse_types::Attestation<T>> for AttestationModel {
    fn from(attestation: &lighthouse_types::Attestation<T>) -> Self {
        AttestationModel {
            slot: attestation.data.slot.as_u64(),
            aggregation_bits: attestation.aggregation_bits.iter().collect(),
            committee_index: attestation.data.index,
            beacon_block_root: attestation.data.beacon_block_root.to_string(),
            source: attestation.data.source.epoch.as_u64(),
            target: attestation.data.target.epoch.as_u64(),
            signature: attestation.signature.to_string(),
        }
    }
}
