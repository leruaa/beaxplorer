use std::{
    fs,
    hash::{Hash, Hasher},
    path::PathBuf,
    str::FromStr,
};

use serde::de::DeserializeOwned;

use crate::{
    path::{FromPath, ToPath},
    persistable::{Persistable, ResolvablePersistable},
};

pub struct ModelWithId<Id, M> {
    pub id: Id,
    pub model: M,
}

impl<Id, M> ModelWithId<Id, M> {
    pub fn new(id: Id, model: M) -> Self {
        Self { id, model }
    }
}

impl<Id, M> ResolvablePersistable for ModelWithId<Id, M>
where
    M: Persistable + ToPath<Id = Id>,
{
    fn save(&self, base_path: &str) -> Result<(), String> {
        let full_path = M::to_path(base_path, &self.id);
        self.model.persist(&full_path)
    }
}

impl<Id, M> ModelWithId<Id, M>
where
    M: DeserializeOwned + ToPath<Id = Id> + FromPath,
    Id: FromStr + Clone,
{
    pub fn iter(base_path: &str) -> Result<impl Iterator<Item = ModelWithId<Id, M>> + '_, String> {
        let path = format!("{}/{}", base_path, M::prefix());

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
                        ModelWithId::new(id.clone(), M::from_path(base_path, &id).unwrap())
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
