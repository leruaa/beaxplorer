use crate::{
    errors::IndexerError,
    persistable::Persistable,
    persistable_collection::{PersistableCollection, PersistableEpochField},
    retriever::Retriever,
};
use types::{
    meta::EpochsMeta,
    views::{BlockView, EpochView},
};

pub struct Indexer {
    epochs: Vec<EpochView>,
    blocks: Vec<BlockView>,
    sorted_epochs_by_fields: Vec<PersistableEpochField>,
}

impl Indexer {
    pub fn index(self, base_dir: &str) -> Result<(), IndexerError> {
        for mut persistable in self.sorted_epochs_by_fields {
            persistable.append(&self.epochs);
            persistable.persist(base_dir)
        }

        EpochsMeta::new(self.epochs.len()).persist(base_dir);

        for epoch in self.epochs {
            epoch.persist(base_dir);
        }

        for block in self.blocks {
            block.persist(base_dir);
        }

        Ok(())
    }
}

impl From<Retriever> for Indexer {
    fn from(retriever: Retriever) -> Self {
        Indexer {
            epochs: retriever.epochs,
            blocks: retriever.blocks,
            sorted_epochs_by_fields: PersistableEpochField::build(),
        }
    }
}
