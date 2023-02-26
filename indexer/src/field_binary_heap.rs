use std::{collections::BinaryHeap, fs::File, io::BufWriter};

use rmp_serde::Serializer;
use serde::Serialize;
use types::persistable::Persistable;

use crate::{orderable::Orderable, persistable_fields::PersistableField};

pub struct FieldBinaryHeap<F: PersistableField<M>, M> {
    inner: BinaryHeap<Orderable<F::Field>>,
}

impl<F: PersistableField<M>, M> FieldBinaryHeap<F, M> {
    pub fn from_model(model: &[M]) -> Self {
        let mut heap = Self::default();
        heap.inner.extend(model.iter().map(|x| F::get_value(x)));
        heap
    }
}

impl<F: PersistableField<M>, M> Default for FieldBinaryHeap<F, M> {
    fn default() -> Self {
        FieldBinaryHeap {
            inner: BinaryHeap::new(),
        }
    }
}

impl<F: PersistableField<M>, M> Persistable for FieldBinaryHeap<F, M> {
    fn persist(self, base_dir: &str) {
        for (i, chunk) in self.inner.into_sorted_vec().chunks(10).enumerate() {
            let indexes: Vec<u64> = chunk.iter().map(|x| x.id).collect();
            let mut f = BufWriter::new(
                File::create(format!("{}/s/{}/{}.msg", base_dir, F::FIELD_NAME, i + 1)).unwrap(),
            );
            indexes.serialize(&mut Serializer::new(&mut f)).unwrap();
        }
    }
}
