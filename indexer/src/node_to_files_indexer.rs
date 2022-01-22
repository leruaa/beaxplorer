use types::{
    block::{BlockModel, BlocksMeta},
    epoch::{EpochModel, EpochsMeta},
    validator::{ValidatorModel, ValidatorsMeta},
};

use crate::{
    errors::IndexerError,
    persistable::Persistable,
    persistable_collection::{PersistableCollection, PersistableEpochField},
    retriever::Retriever,
};

pub struct Indexer {
    epochs: Vec<EpochModel>,
    blocks: Vec<BlockModel>,
    validators: Vec<ValidatorModel>,
    sorted_epochs_by_fields: Vec<PersistableEpochField>,
}

impl Indexer {
    pub fn index(self, base_dir: &str) -> Result<(), IndexerError> {
        for mut persistable in self.sorted_epochs_by_fields {
            persistable.append(&self.epochs);
            persistable.persist(&format!("{}/epochs", base_dir))
        }

        EpochsMeta::new(self.epochs.len()).persist(base_dir);

        for epoch in self.epochs {
            epoch.persist(base_dir);
        }

        BlocksMeta::new(self.blocks.len()).persist(base_dir);

        for block in self.blocks {
            block.persist(base_dir);
        }

        ValidatorsMeta::new(self.validators.len()).persist(base_dir);

        self.validators.persist(base_dir);

        Ok(())
    }
}

impl From<Retriever> for Indexer {
    fn from(retriever: Retriever) -> Self {
        Indexer {
            epochs: retriever.epochs,
            blocks: retriever.blocks,
            validators: retriever.validators,
            sorted_epochs_by_fields: PersistableEpochField::build(),
        }
    }
}
