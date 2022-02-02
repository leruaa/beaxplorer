use lighthouse_types::MainnetEthSpec;
use types::{
    block::{BlockExtendedModelWithId, BlockModelWithId, BlocksMeta},
    commitee::CommiteesModelWithId,
    epoch::{EpochExtendedModelWithId, EpochModelWithId, EpochsMeta},
    validator::{ValidatorModelWithId, ValidatorsMeta},
};

use crate::{
    errors::IndexerError,
    persistable::Persistable,
    persistable_collection::{PersistableCollection, PersistableEpochField},
    retriever::Retriever,
    types::{
        consolidated_block::ConsolidatedBlock, consolidated_epoch::ConsolidatedEpoch,
        consolidated_validator::ConsolidatedValidator,
    },
};

pub struct Indexer {
    epochs: Vec<ConsolidatedEpoch<MainnetEthSpec>>,
    validators: Vec<ConsolidatedValidator>,
    sorted_epochs_by_fields: Vec<PersistableEpochField>,
}

impl Indexer {
    pub fn index(self, base_dir: &str) -> Result<(), IndexerError> {
        let (epochs, extended_epochs) = self
            .epochs
            .iter()
            .map(|x| (EpochModelWithId::from(x), EpochExtendedModelWithId::from(x)))
            .unzip::<EpochModelWithId, EpochExtendedModelWithId, Vec<EpochModelWithId>, Vec<EpochExtendedModelWithId>>();

        let all_blocks = self
            .epochs
            .into_iter()
            .flat_map(|x| x.blocks)
            .collect::<Vec<ConsolidatedBlock<MainnetEthSpec>>>();

        let (blocks, extended_blocks) = all_blocks
            .iter()
            .map(|x| (BlockModelWithId::from(x), BlockExtendedModelWithId::from(x)))
            .unzip::<BlockModelWithId, BlockExtendedModelWithId, Vec<BlockModelWithId>, Vec<BlockExtendedModelWithId>>();

        let committees = all_blocks
            .iter()
            .map(|x| CommiteesModelWithId::from(x))
            .collect::<Vec<CommiteesModelWithId>>();

        let validators = self
            .validators
            .iter()
            .map(ValidatorModelWithId::from)
            .collect::<Vec<ValidatorModelWithId>>();

        for mut persistable in self.sorted_epochs_by_fields {
            persistable.append(&epochs);
            persistable.persist(&format!("{}/epochs", base_dir))
        }

        EpochsMeta::new(epochs.len()).persist(base_dir);

        epochs.persist(base_dir);
        extended_epochs.persist(base_dir);

        BlocksMeta::new(blocks.len()).persist(base_dir);

        blocks.persist(base_dir);
        extended_blocks.persist(base_dir);
        committees.persist(base_dir);

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
            sorted_epochs_by_fields: PersistableEpochField::build(),
        }
    }
}
