use std::{collections::BinaryHeap, fs::File, io::BufWriter};

use rmp_serde::Serializer;
use serde::Serialize;

use super::Orderable;

pub struct FieldBinaryHeap<F: Ord> {
    inner: BinaryHeap<Orderable<F>>,
}

impl<F: Ord> FieldBinaryHeap<F> {
    pub fn new() -> Self {
        FieldBinaryHeap {
            inner: BinaryHeap::new(),
        }
    }

    pub fn from_orderables(iter: impl Iterator<Item = Orderable<F>>) -> Self {
        let mut heap = Self::new();
        heap.inner.extend(iter);
        heap
    }

    pub fn push(&mut self, item: Orderable<F>) {
        self.inner.push(item);
    }

    pub fn persist(self, base_dir: &str, field_name: &str) {
        for (i, chunk) in self.inner.into_sorted_vec().chunks(10).enumerate() {
            let indexes: Vec<u64> = chunk.iter().map(|x| x.id).collect();
            let mut f = BufWriter::new(
                File::create(format!("{}/s/{}/{}.msg", base_dir, field_name, i + 1)).unwrap(),
            );
            indexes.serialize(&mut Serializer::new(&mut f)).unwrap();
        }
    }
}
