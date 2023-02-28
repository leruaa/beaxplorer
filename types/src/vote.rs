use crate::model::ModelWithId;
use indexer_macro::Persistable;
use indexer_macro::ToPathWithId;
use serde::Deserialize;
use serde::Serialize;

#[derive(Persistable, ToPathWithId, Serialize, Deserialize, Debug, Clone)]
#[persistable(index = "collection")]
#[to_path(prefix = "/blocks/v")]
pub struct VoteModel {
    pub slot: u64,
    pub committee_index: u64,
}

#[cfg(feature = "indexing")]
impl<E: lighthouse_types::EthSpec> From<&lighthouse_types::Attestation<E>> for VoteModel {
    fn from(value: &lighthouse_types::Attestation<E>) -> Self {
        VoteModel {
            slot: value.data.slot.as_u64(),
            committee_index: value.data.index,
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
            model: value.1.iter().map(VoteModel::from).collect::<Vec<_>>(),
        }
    }
}
