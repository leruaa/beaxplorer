use types::{Epoch, MainnetEthSpec};

use crate::epoch_retriever::EpochRetriever;

pub struct Indexer {
    epoch_retriever: EpochRetriever
}

impl Indexer {
    pub fn new() -> Self {
        Indexer {
            epoch_retriever: EpochRetriever::new(),
        }
    }

    pub fn index(&self) {
        let consolidated_epoch = &self.epoch_retriever.get_consolidated_epoch::<MainnetEthSpec>(Epoch::new(40000));
    }
}