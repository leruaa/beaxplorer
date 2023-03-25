use std::collections::{hash_map::Entry, HashMap};

use lighthouse_types::{Epoch, EthSpec, Slot};

use crate::types::block_state::BlockState;

#[derive(Debug, Default)]
pub struct BlocksByEpoch<E: EthSpec>(HashMap<Epoch, HashMap<Slot, BlockState<E>>>);

impl<E: EthSpec> BlocksByEpoch<E> {
    pub fn build_epoch(&mut self, block: BlockState<E>) -> Option<EpochToPersist<E>> {
        let slot = match &block {
            BlockState::Proposed(block) => block.message().slot(),
            BlockState::Orphaned(block) => block.message().slot(),
            BlockState::Missed(slot) => *slot,
        };

        let epoch = slot.epoch(E::slots_per_epoch());

        let blocks_by_slot = self.0.entry(epoch).or_insert_with(HashMap::new);

        match blocks_by_slot.entry(slot) {
            Entry::Occupied(mut e) => {
                if let BlockState::Missed(_) = e.get() {
                    e.insert(block);
                }
            }
            Entry::Vacant(e) => {
                e.insert(block);
            }
        };

        match self.0.entry(epoch) {
            Entry::Occupied(e) => {
                if e.get().len() as u64 == E::slots_per_epoch() {
                    Some(EpochToPersist::new(epoch, e.remove()))
                } else {
                    None
                }
            }
            Entry::Vacant(_) => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EpochToPersist<E: EthSpec> {
    pub epoch: Epoch,
    pub blocks: HashMap<Slot, BlockState<E>>,
}

impl<E: EthSpec> EpochToPersist<E> {
    pub fn new(epoch: Epoch, blocks: HashMap<Slot, BlockState<E>>) -> Self {
        Self { epoch, blocks }
    }
}
