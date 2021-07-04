use std::collections::HashMap;

use lighthouse_types::{Epoch, EthSpec, Slot, Validator};

use super::consolidated_block::ConsolidatedBlock;

#[derive(Debug)]
pub struct ConsolidatedEpoch<E: EthSpec> {
    pub epoch: Epoch,
    pub blocks: HashMap<Slot, ConsolidatedBlock<E>>,
    pub validators: Vec<Validator>,
}

impl<E: EthSpec> ConsolidatedEpoch<E> {
    pub fn new(epoch: Epoch) -> Self {
        ConsolidatedEpoch {
            epoch,
            blocks: HashMap::new(),
            validators: Vec::new(),
        }
    }
}