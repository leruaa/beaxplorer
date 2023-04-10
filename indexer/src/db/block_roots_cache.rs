use std::num::NonZeroUsize;

use lighthouse_types::{Hash256, Slot};
use lru::LruCache;
use types::{block_root::BlockRootModel, path::FromPath};

#[derive(Debug)]
pub struct BlockRootsCache {
    base_dir: String,
    block_roots_cache: LruCache<Hash256, Slot>,
}

impl BlockRootsCache {
    pub fn new(base_dir: String) -> Self {
        Self {
            base_dir,
            block_roots_cache: LruCache::new(NonZeroUsize::new(64).unwrap()),
        }
    }

    pub fn put(&mut self, root: Hash256, slot: Slot) {
        self.block_roots_cache.put(root, slot);
    }

    pub fn get(&mut self, root: Hash256) -> Option<Slot> {
        if let Some(slot) = self.block_roots_cache.get(&root) {
            Some(slot.to_owned())
        } else if let Ok(model) = BlockRootModel::from_path(&self.base_dir, &format!("{root:?}")) {
            let slot = Slot::new(model.slot);
            self.block_roots_cache.put(root, slot);
            Some(slot)
        } else {
            None
        }
    }

    pub fn contains(&mut self, root: Hash256) -> bool {
        self.get(root).is_some()
    }
}
