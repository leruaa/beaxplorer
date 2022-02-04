use lighthouse_types::MainnetEthSpec;
use types::{
    attestation::AttestationsModelWithId,
    block::{BlockExtendedModelWithId, BlockModelWithId, BlocksMeta},
    committee::CommitteesModelWithId,
    epoch::{EpochExtendedModelWithId, EpochModelWithId, EpochsMeta},
    validator::{ValidatorModelWithId, ValidatorsMeta},
};

use crate::{
    errors::IndexerError,
    field_binary_heap::FieldBinaryHeap,
    persistable::Persistable,
    persistable_fields::EpochDepositCount,
    retriever::Retriever,
    types::{consolidated_epoch::ConsolidatedEpoch, consolidated_validator::ConsolidatedValidator},
};

pub struct Indexer {
    epochs: Vec<ConsolidatedEpoch<MainnetEthSpec>>,
    validators: Vec<ConsolidatedValidator>,
}

impl Indexer {
    pub fn index(self, base_dir: &str) -> Result<(), IndexerError> {
        let epochs = self
            .epochs
            .iter()
            .map(EpochModelWithId::from)
            .collect::<Vec<_>>();

        let epochs_extended = self
            .epochs
            .iter()
            .map(EpochExtendedModelWithId::from)
            .collect::<Vec<_>>();

        let all_blocks = self
            .epochs
            .into_iter()
            .flat_map(|x| x.blocks)
            .collect::<Vec<_>>();

        let blocks = all_blocks
            .iter()
            .map(BlockModelWithId::from)
            .collect::<Vec<_>>();

        let extended_blocks = all_blocks
            .iter()
            .map(BlockExtendedModelWithId::from)
            .collect::<Vec<_>>();

        let committees = all_blocks
            .iter()
            .map(CommitteesModelWithId::from)
            .collect::<Vec<_>>();

        let attestations = all_blocks
            .iter()
            .map(AttestationsModelWithId::from)
            .collect::<Vec<_>>();

        let validators = self
            .validators
            .iter()
            .map(ValidatorModelWithId::from)
            .collect::<Vec<_>>();

        EpochsMeta::new(epochs.len()).persist(base_dir);

        FieldBinaryHeap::<EpochDepositCount>::from_model(&epochs).persist(base_dir);

        epochs.persist(base_dir);
        epochs_extended.persist(base_dir);

        BlocksMeta::new(blocks.len()).persist(base_dir);

        blocks.persist(base_dir);
        extended_blocks.persist(base_dir);
        committees.persist(base_dir);
        attestations.persist(base_dir);

        ValidatorsMeta::new(self.validators.len()).persist(base_dir);

        validators.persist(base_dir);

        Ok(())
    }
}

impl From<Retriever> for Indexer {
    fn from(retriever: Retriever) -> Self {
        Indexer {
            epochs: retriever.epochs,
            validators: retriever.validators,
        }
    }
}
