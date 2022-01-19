use crate::{
    field_binary_heap::FieldBinaryHeap, indexable::Indexable, persistable::Persistable,
    persistable_collection::PersistableCollection,
};
use rmp_serde::Serializer;
use serde::Serialize;
use std::{fs::File, io::BufWriter};

pub struct PersistableBinaryHeap<I: Indexable, T: Ord + Eq + Clone> {
    inner: FieldBinaryHeap<I, T>,
    field_name: String,
}

impl<I: Indexable, T: Ord + Eq + Clone> PersistableBinaryHeap<I, T> {
    pub fn new<F: Fn(&I) -> T + Send + 'static>(field_name: String, get_field_value_fn: F) -> Self {
        PersistableBinaryHeap {
            inner: FieldBinaryHeap::new(get_field_value_fn),
            field_name,
        }
    }
}

impl<I: Indexable, T: Ord + Eq + Clone + Send> PersistableCollection<I>
    for PersistableBinaryHeap<I, T>
{
    fn insert(&mut self, indexable: &I) {
        self.inner.push(indexable)
    }
}

impl<I: Indexable, T: Ord + Eq + Clone + Send> Persistable for PersistableBinaryHeap<I, T> {
    fn persist(self, base_dir: &str) -> () {
        for (i, chunk) in self.inner.into_sorted_vec().chunks(10).enumerate() {
            let indexes: Vec<u64> = chunk.into_iter().map(|x| x.epoch).collect();
            let mut f = BufWriter::new(
                File::create(format!("{}/s/{}/{}.msg", base_dir, self.field_name, i + 1)).unwrap(),
            );
            indexes.serialize(&mut Serializer::new(&mut f)).unwrap();
        }
    }
}
