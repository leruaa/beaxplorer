use std::{collections::HashSet, num::NonZeroUsize};

use lighthouse_types::Slot;
use lru::LruCache;
use types::{model::ModelWithId, path::FromPath, vote::VoteModelsWithId};

#[derive(Debug)]
pub struct VotesCache {
    base_dir: String,
    votes_cache: LruCache<Slot, VoteModelsWithId>,
    dirty_slots: HashSet<Slot>,
}

impl VotesCache {
    pub fn new(base_dir: String) -> Self {
        Self {
            base_dir,
            votes_cache: LruCache::new(NonZeroUsize::new(64).unwrap()),
            dirty_slots: HashSet::new(),
        }
    }

    pub fn get_mut(&mut self, slot: Slot) -> &mut VoteModelsWithId {
        let base_dir = self.base_dir.clone();

        self.dirty_slots.insert(slot);

        self.votes_cache.get_or_insert_mut(slot, || ModelWithId {
            id: slot.as_u64(),
            model: VoteModelsWithId::from_path(&base_dir, &slot.as_u64()).unwrap_or_default(),
        })
    }

    pub fn drain_dirty<F: FnMut(&VoteModelsWithId)>(&mut self, mut f: F) {
        self.dirty_slots.iter().for_each(|s| {
            if let Some(votes) = self.votes_cache.peek(s) {
                f(votes)
            }
        });

        self.dirty_slots.clear();
    }
}
