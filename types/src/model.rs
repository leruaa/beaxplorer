use serde::Serialize;

use crate::path::{AsPath, ToPath};

pub struct ModelWithId<M: Serialize + Send> {
    pub id: u64,
    pub model: M,
}

impl<T> AsPath for ModelWithId<T>
where
    T: Serialize + Send,
    ModelWithId<T>: ToPath<u64>,
{
    fn as_path(&self, base: &str) -> String {
        Self::to_path(base, self.id)
    }
}
