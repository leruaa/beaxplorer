use std::fs::File;

use serde::{de::DeserializeOwned, Serialize};

use crate::path::{AsPath, ToPath};

#[cfg(feature = "indexing")]
use rmp_serde;

pub struct ModelWithId<M: Serialize + Send> {
    pub id: u64,
    pub model: M,
}

impl<T> AsPath for ModelWithId<T>
where
    T: Serialize + Send,
    ModelWithId<T>: ToPath,
{
    fn as_path(&self, base: &str) -> String {
        Self::to_path(base, self.id)
    }
}

#[cfg(feature = "indexing")]
impl<T> ModelWithId<T>
where
    T: Serialize + DeserializeOwned + Send,
    ModelWithId<T>: ToPath,
{
    pub fn from_path(base_path: &str, id: u64) -> T {
        let path = Self::to_path(base_path, id);
        let file = File::open(path).unwrap();
        rmp_serde::from_read::<_, T>(file).unwrap()
    }
}
