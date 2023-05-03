use indexer_macro::Persistable;
use serde::Deserialize;
use serde::Serialize;
#[cfg(feature = "wasm")]
use tsify::Tsify;

#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[persistable(model = "collection")]
#[persistable(prefix = "/blocks/a")]
#[serde(rename_all = "camelCase")]
pub struct AttestationModel {
    pub slot: u64,
    pub aggregation_bits: Vec<bool>,
    pub committee_index: u64,
    pub beacon_block_root: String,
    pub source_epoch: u64,
    pub source_root: String,
    pub target_epoch: u64,
    pub target_root: String,
    pub signature: String,
}

#[cfg(feature = "indexing")]
impl<T: lighthouse_types::EthSpec> From<&lighthouse_types::Attestation<T>> for AttestationModel {
    fn from(attestation: &lighthouse_types::Attestation<T>) -> Self {
        AttestationModel {
            slot: attestation.data.slot.as_u64(),
            aggregation_bits: attestation.aggregation_bits.iter().collect(),
            committee_index: attestation.data.index,
            beacon_block_root: format!("{:?}", attestation.data.beacon_block_root),
            source_epoch: attestation.data.source.epoch.as_u64(),
            source_root: format!("{:?}", attestation.data.source.root),
            target_epoch: attestation.data.target.epoch.as_u64(),
            target_root: format!("{:?}", attestation.data.target.root),
            signature: attestation.signature.to_string(),
        }
    }
}
