use std::{
    fs,
    hash::{Hash, Hasher},
    path::PathBuf,
    str::FromStr,
};

use serde::de::DeserializeOwned;

use crate::path::{FromPath, ToPath};

pub struct ModelWithId<Id, M> {
    pub id: Id,
    pub model: M,
}

impl<Id, M> ModelWithId<Id, M> {
    pub fn new(id: Id, model: M) -> Self {
        Self { id, model }
    }
}

impl<Id, T> ModelWithId<Id, T>
where
    T: DeserializeOwned + ToPath<Id = Id>,
    Id: FromStr + Clone,
{
    pub fn iter(base_path: &str) -> Result<impl Iterator<Item = ModelWithId<Id, T>> + '_, String> {
        let path = format!("{}/{}", base_path, T::prefix());

        fs::read_dir(path)
            .map(|r| {
                r.filter_map(|dir| dir.ok())
                    .filter(|dir| dir.file_type().map_or(false, |f| f.is_file()))
                    .filter_map(|dir| dir.file_name().into_string().ok())
                    .filter(|file_name| {
                        file_name.ends_with(".msg") && !file_name.starts_with("meta")
                    })
                    .map(|file_name| file_name.replace(".msg", ""))
                    .filter_map(|id| id.replace(".msg", "").parse::<Id>().ok())
                    .map(move |id| {
                        ModelWithId::new(id.clone(), T::from_path(base_path, &id).unwrap())
                    })
            })
            .map_err(|err| err.to_string())
    }
}

impl<Id, M> ToPath for ModelWithId<Id, M>
where
    M: ToPath,
{
    type Id = M::Id;

    fn prefix() -> String {
        M::prefix()
    }

    fn to_path(base_dir: &str, id: &Self::Id) -> String {
        M::to_path(base_dir, id)
    }

    fn dirs(base_dir: &str) -> Vec<PathBuf> {
        M::dirs(base_dir)
    }
}

impl<M> FromPath for ModelWithId<M::Id, M>
where
    M: ToPath + DeserializeOwned,
{
    type Id = M::Id;
    type Model = M;

    fn from_path(base_dir: &str, id: &M::Id) -> Result<M, String> {
        M::from_path(base_dir, id)
    }
}

impl<Id, M> PartialEq for ModelWithId<Id, M>
where
    Id: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<Id, M> Eq for ModelWithId<Id, M> where Id: PartialEq {}

impl<Id, M> Hash for ModelWithId<Id, M>
where
    Id: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
