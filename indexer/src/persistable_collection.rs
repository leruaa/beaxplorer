use types::views::EpochView;

use crate::{persistable::Persistable, persistable_binary_heap::PersistableBinaryHeap};

pub trait PersistableCollection<I>: Persistable {
    fn insert(&mut self, indexable: &I) -> ();

    fn append(&mut self, indexables: &Vec<I>) -> () {
        for i in indexables {
            self.insert(i)
        }
    }
}

pub enum PersistableEpochField {
    AsUsize(PersistableBinaryHeap<EpochView, usize>),
}

impl PersistableEpochField {
    pub fn build() -> Vec<Self> {
        vec![
            PersistableEpochField::AsUsize(PersistableBinaryHeap::new(
                "attestations_count".to_string(),
                |e: &EpochView| e.attestations_count,
            )),
            PersistableEpochField::AsUsize(PersistableBinaryHeap::new(
                "deposits_count".to_string(),
                |e: &EpochView| e.deposits_count,
            )),
        ]
    }
}

impl Persistable for PersistableEpochField {
    fn persist(self, base_dir: &str) -> () {
        match self {
            PersistableEpochField::AsUsize(p) => p.persist(base_dir),
        }
    }
}

impl PersistableCollection<EpochView> for PersistableEpochField {
    fn insert(&mut self, indexable: &EpochView) -> () {
        match self {
            PersistableEpochField::AsUsize(p) => p.insert(indexable),
        }
    }
}
