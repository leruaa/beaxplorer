use types::epoch::EpochModel;

use crate::{persistable::Persistable, persistable_binary_heap::PersistableBinaryHeap};

pub trait PersistableCollection<I>: Persistable {
    fn insert(&mut self, indexable: &I, id: &u64) -> ();

    fn append(&mut self, indexables: &Vec<(u64, I)>) -> () {
        for (id, indexable) in indexables {
            self.insert(indexable, id)
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
    fn insert(&mut self, indexable: &EpochModel, id: &u64) -> () {
        match self {
            PersistableEpochField::AsUsize(p) => p.insert(indexable, id),
        }
    }
}
