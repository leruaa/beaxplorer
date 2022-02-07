use lighthouse_types::AggregateSignature;
use lighthouse_types::Attestation;
use lighthouse_types::Checkpoint;
use lighthouse_types::EthSpec;
use lighthouse_types::Hash256;
use serde::Deserialize;
use serde::Serialize;

use crate::model::ModelWithId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttestationModel {
    pub aggregation_bits: Vec<bool>,
    pub committee_index: u64,
    pub beacon_block_root: Hash256,
    pub source: Checkpoint,
    pub target: Checkpoint,
    pub signature: AggregateSignature,
}

pub type AttestationsModelWithId = ModelWithId<Vec<AttestationModel>>;

impl<T: EthSpec> From<&Attestation<T>> for AttestationModel {
    fn from(attestation: &Attestation<T>) -> Self {
        AttestationModel {
            aggregation_bits: attestation.aggregation_bits.iter().collect(),
            committee_index: attestation.data.index,
            beacon_block_root: attestation.data.beacon_block_root,
            source: attestation.data.source,
            target: attestation.data.target,
            signature: attestation.signature.clone(),
        }
    }
}
