use std::{collections::BinaryHeap, marker::PhantomData};

use crate::{indexable::Indexable, orderable::Orderable};

pub struct FieldBinaryHeap<T: Ord + Eq + Clone, I: Indexable> {
    inner: BinaryHeap<Orderable<T>>,
    get_field_value_fn: Box<dyn Fn(&I) -> T + Send>,
    phantom: PhantomData<I>,
}

impl<T: Ord + Eq + Clone, I: Indexable> FieldBinaryHeap<T, I> {
    pub fn new<F: Fn(&I) -> T + Send + 'static>(get_field_value_fn: F) -> Self {
        FieldBinaryHeap {
            inner: BinaryHeap::new(),
            get_field_value_fn: Box::new(get_field_value_fn),
            phantom: PhantomData,
        }
    }

    pub fn push(&mut self, view: &I) {
        self.inner.push(Orderable::from((
            view.get_id().clone(),
            (self.get_field_value_fn)(view).clone(),
        )))
    }

    pub fn into_sorted_vec(self) -> Vec<Orderable<T>> {
        self.inner.into_sorted_vec()
    }
}
