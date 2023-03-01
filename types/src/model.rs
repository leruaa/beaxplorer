use std::fs::File;

use serde::{de::DeserializeOwned, Serialize};

use crate::path::{FromPath, ToPath};

#[cfg(feature = "indexing")]
use rmp_serde;

pub struct ModelWithId<M> {
    pub id: u64,
    pub model: M,
}

impl<T> ToPath<u64> for ModelWithId<T>
where
    T: ToPath<u64>,
{
    fn to_path(base_dir: &str, id: u64) -> String {
        T::to_path(base_dir, id)
    }
}

impl<M> FromPath<M, u64> for ModelWithId<M>
where
    M: ToPath<u64> + DeserializeOwned,
{
    fn from_path(base_dir: &str, id: u64) -> M {
        let path = M::to_path(base_dir, id);
        let file = std::fs::File::open(path).unwrap();
        rmp_serde::from_read::<_, M>(file).unwrap()
    }
}
