use std::collections::HashSet;

use lighthouse_types::Hash256;

#[derive(Debug, Default)]
pub struct ProposedBlockRoots(HashSet<Hash256>);

impl ProposedBlockRoots {
    pub fn contains(&self, root: &Hash256) -> bool {
        self.0.contains(root)
    }

    pub fn insert(&mut self, root: Hash256) -> bool {
        self.0.insert(root)
    }
}
