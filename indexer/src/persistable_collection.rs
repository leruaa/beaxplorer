use types::epoch::EpochModel;

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
    AsUsize(PersistableBinaryHeap<EpochModel, usize>),
}

impl PersistableEpochField {
    pub fn build() -> Vec<Self> {
        vec![
            PersistableEpochField::AsUsize(PersistableBinaryHeap::new(
                "attestations_count".to_string(),
                |e: &EpochModel| e.attestations_count,
            )),
            PersistableEpochField::AsUsize(PersistableBinaryHeap::new(
                "deposits_count".to_string(),
                |e: &EpochModel| e.deposits_count,
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

impl PersistableCollection<EpochModel> for PersistableEpochField {
    fn insert(&mut self, indexable: &EpochModel) -> () {
        match self {
            PersistableEpochField::AsUsize(p) => p.insert(indexable),
        }
    }
}
