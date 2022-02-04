use std::{collections::BinaryHeap, fs::File, io::BufWriter};

use rmp_serde::Serializer;
use serde::Serialize;

use crate::{orderable::Orderable, persistable::Persistable, persistable_fields::PersistableField};

pub struct FieldBinaryHeap<F: PersistableField> {
    inner: BinaryHeap<Orderable<F::Field>>,
}

impl<F: PersistableField> FieldBinaryHeap<F> {
    pub fn new() -> Self {
        FieldBinaryHeap {
            inner: BinaryHeap::new(),
        }
    }

    pub fn from_model(model: &Vec<F::Model>) -> Self {
        let mut heap = Self::new();
        heap.inner.extend(model.iter().map(|x| F::get_value(x)));
        heap
    }
}

impl<F: PersistableField> Persistable for FieldBinaryHeap<F> {
    fn persist(self, base_dir: &str) -> () {
        for (i, chunk) in self.inner.into_sorted_vec().chunks(10).enumerate() {
            let indexes: Vec<u64> = chunk.into_iter().map(|x| x.id).collect();
            let mut f = BufWriter::new(
                File::create(format!(
                    "{}/s/{}/{}.msg",
                    base_dir,
                    F::get_field_name(),
                    i + 1
                ))
                .unwrap(),
            );
            indexes.serialize(&mut Serializer::new(&mut f)).unwrap();
        }
    }
}
