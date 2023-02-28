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
    T: DeserializeOwned + ToPath<u64>,
{
    pub fn from_path(base_path: &str, id: u64) -> T {
        let path = T::to_path(base_path, id);
        let file = File::open(path).unwrap();
        rmp_serde::from_read::<_, T>(file).unwrap()
    }
}

impl<T, Id> ToPath<Id> for ModelWithId<T>
where
    T: ToPath<Id>,
{
    fn to_path(base_dir: &str, id: Id) -> String {
        T::to_path(base_dir, id)
    }
}