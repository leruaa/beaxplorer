use serde::de::DeserializeOwned;

use crate::{
    meta::{Meta, WithMeta},
    path::{FromPath, ToPath},
};

#[cfg(feature = "indexing")]
use rmp_serde;

pub struct ModelWithId<M> {
    pub id: u64,
    pub model: M,
}

impl<M> ModelWithId<M> {
    pub fn new(id: u64, model: M) -> Self {
        Self { id, model }
    }
}

impl<T> ModelWithId<T>
where
    T: DeserializeOwned + ToPath<u64> + WithMeta,
    <T as WithMeta>::MetaType: Meta,
{
    pub fn all(base_path: &str) -> Vec<ModelWithId<T>> {
        let meta = T::meta(base_path);
        let mut all_models = vec![];

        for i in 1..meta.count() {
            let id = i as u64;
            let m = T::from_path(base_path, &id);
            all_models.push(ModelWithId::new(id, m))
        }

        all_models
    }
}

impl<T> ToPath<u64> for ModelWithId<T>
where
    T: ToPath<u64>,
{
    fn prefix() -> String {
        T::prefix()
    }

    fn to_path(base_dir: &str, id: &Id) -> String {
        T::to_path(base_dir, id)
    }
}

impl<M> FromPath<M, u64> for ModelWithId<M>
where
    M: ToPath<u64> + DeserializeOwned,
{
    fn from_path(base_dir: &str, id: &Id) -> M {
        let path = M::to_path(base_dir, &id);
        let file = std::fs::File::open(path).unwrap();
        rmp_serde::from_read::<_, M>(file).unwrap()
    }
}
