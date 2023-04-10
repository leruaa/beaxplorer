use crate::model::ModelWithId;
use indexer_macro::Persistable;
use serde::Deserialize;
use serde::Serialize;

#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[persistable(model = "collection")]
#[persistable(prefix = "/blocks/v")]
pub struct VoteModel {
    pub slot: u64,
    pub committee_index: u64,
}

#[cfg(feature = "indexing")]
impl From<&lighthouse_types::AttestationData> for VoteModel {
    fn from(value: &lighthouse_types::AttestationData) -> Self {
        VoteModel {
            slot: value.slot.as_u64(),
            committee_index: value.index,
        }
    }
}

#[cfg(feature = "indexing")]
impl<E: lighthouse_types::EthSpec>
    From<(
        &lighthouse_types::Slot,
        Vec<lighthouse_types::Attestation<E>>,
    )> for VoteModelsWithId
{
    fn from(
        value: (
            &lighthouse_types::Slot,
            Vec<lighthouse_types::Attestation<E>>,
        ),
    ) -> Self {
        VoteModelsWithId {
            id: value.0.as_u64(),
            model: value
                .1
                .iter()
                .map(|a| VoteModel::from(&a.data))
                .collect::<Vec<_>>(),
        }
    }
}
