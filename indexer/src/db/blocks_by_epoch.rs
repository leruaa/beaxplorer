use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};

use lighthouse_types::{Epoch, EthSpec, Slot};
use parking_lot::RwLock;

use crate::{types::block_state::BlockState, work::Work};

use super::latest_epoch::LatestEpoch;

#[derive(Default, Debug)]
pub struct BlocksByEpoch<E: EthSpec> {
    latest_epoch: Arc<RwLock<LatestEpoch>>,
    blocks_by_epoch: HashMap<Epoch, HashMap<Slot, BlockState<E>>>,
}

impl<E: EthSpec> BlocksByEpoch<E> {
    pub fn new(latest_epoch: Arc<RwLock<LatestEpoch>>) -> Self {
        Self {
            latest_epoch,
            ..Default::default()
        }
    }

    pub fn build_epoch(&mut self, block: BlockState<E>) -> Option<Work<E>> {
        let slot = match &block {
            BlockState::Proposed(block) => block.message().slot(),
            BlockState::Orphaned(block) => block.message().slot(),
            BlockState::Missed(slot) => *slot,
        };

        let epoch = slot.epoch(E::slots_per_epoch());
        let current_epoch = self
            .latest_epoch
            .read()
            .map(|e| e.as_u64() + 1)
            .unwrap_or_default();

        if epoch >= current_epoch {
            let blocks_by_slot = self
                .blocks_by_epoch
                .entry(epoch)
                .or_insert_with(HashMap::new);

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

            match self.blocks_by_epoch.entry(epoch) {
                Entry::Occupied(e) => {
                    if epoch == current_epoch && e.get().len() as u64 == E::slots_per_epoch() {
                        Some(Work::PersistEpoch {
                            epoch,
                            blocks: e.remove(),
                        })
                    } else {
                        None
                    }
                }
                Entry::Vacant(_) => None,
            }
        } else if let BlockState::Orphaned(block) = block {
            Some(Work::PersistBlock(block))
        } else {
            None
        }
    }
}
