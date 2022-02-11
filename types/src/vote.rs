use crate::model::ModelWithId;
use crate::path::AsPath;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
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

pub type VotesModelWithId = ModelWithId<Vec<VoteModel>>;

#[cfg(feature = "indexing")]
impl<E: lighthouse_types::EthSpec>
    From<(
        &lighthouse_types::Slot,
        Vec<lighthouse_types::Attestation<E>>,
    )> for VotesModelWithId
{
    fn from(
        value: (
            &lighthouse_types::Slot,
            Vec<lighthouse_types::Attestation<E>>,
        ),
    ) -> Self {
        VotesModelWithId {
            id: value.0.as_u64(),
            model: value.1.iter().map(VoteModel::from).collect::<Vec<_>>(),
        }
    }
}

impl AsPath for VotesModelWithId {
    fn as_path(&self, base: &str) -> String {
        format!("{}/blocks/v/{}.msg", base, self.id)
    }
}
