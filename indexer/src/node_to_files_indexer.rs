use lighthouse_types::MainnetEthSpec;
use types::{
    block::{BlockModel, BlocksMeta},
    epoch::{EpochExtendedModel, EpochModel, EpochsMeta},
    validator::{ValidatorModel, ValidatorsMeta},
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
        let (epochs, _extended_epochs) = self
            .epochs
            .iter()
            .map(|x| (EpochModel::from(x), EpochExtendedModel::from(x)))
            .unzip::<EpochModel, EpochExtendedModel, Vec<EpochModel>, Vec<EpochExtendedModel>>();

        let all_blocks = self
            .epochs
            .into_iter()
            .flat_map(|x| x.blocks)
            .collect::<Vec<ConsolidatedBlock<MainnetEthSpec>>>();

        let blocks = all_blocks
            .iter()
            .map(BlockModel::from)
            .collect::<Vec<BlockModel>>();

        let validators = self
            .validators
            .iter()
            .map(ValidatorModel::from)
            .collect::<Vec<ValidatorModel>>();

        for mut persistable in self.sorted_epochs_by_fields {
            persistable.append(&epochs);
            persistable.persist(&format!("{}/epochs", base_dir))
        }

        EpochsMeta::new(epochs.len()).persist(base_dir);

        epochs.persist(base_dir);

        BlocksMeta::new(blocks.len()).persist(base_dir);

        blocks.persist(base_dir);

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
