use std::fs::File;

use serde::{de::DeserializeOwned, Serialize};

use crate::path::ToPath;

#[cfg(feature = "indexing")]
use rmp_serde;

pub struct ModelWithId<M> {
    pub id: u64,
    pub model: M,
}

#[cfg(feature = "indexing")]
impl<T> ModelWithId<T>
where
    T: Serialize + DeserializeOwned + Send,
    ModelWithId<T>: ToPath<u64>,
{
    pub fn from_path(base_path: &str, id: u64) -> T {
        let path = Self::to_path(base_path, id);
        let file = File::open(path).unwrap();
        rmp_serde::from_read::<_, T>(file).unwrap()
    }
}
