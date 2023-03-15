use std::{fs, str::FromStr};

use serde::de::DeserializeOwned;

use crate::path::{FromPath, ToPath};

#[cfg(feature = "indexing")]
use rmp_serde;

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
    T: DeserializeOwned + ToPath<Id>,
    Id: FromStr,
{
    pub fn all(base_path: &str) -> Result<Vec<ModelWithId<Id, T>>, String> {
        let mut all_models = vec![];
        let path = format!("{}/{}", base_path, T::prefix());

        for entry in fs::read_dir(path).map_err(|err| err.to_string())? {
            let entry = entry.map_err(|err| err.to_string())?;

            if entry.file_type().map_or(false, |f| f.is_file()) {
                if let Ok(file_name) = entry.file_name().into_string() {
                    if file_name.ends_with(".msg") && !file_name.starts_with("meta") {
                        let id_as_string = file_name.replace(".msg", "");
                        let id = id_as_string
                            .replace(".msg", "")
                            .parse::<Id>()
                            .map_err(|_| format!("Failed to parse '{}'", id_as_string))?;
                        let m = T::from_path(base_path, &id);
                        all_models.push(ModelWithId::new(id, m))
                    }
                }
            }
        }

        Ok(all_models)
    }
}

impl<T, Id> ToPath<Id> for ModelWithId<Id, T>
where
    T: ToPath<Id>,
{
    fn prefix() -> String {
        T::prefix()
    }

    fn to_path(base_dir: &str, id: &Id) -> String {
        T::to_path(base_dir, id)
    }
}

impl<Id, M> FromPath<Id, M> for ModelWithId<Id, M>
where
    M: ToPath<Id> + DeserializeOwned,
{
    fn from_path(base_dir: &str, id: &Id) -> M {
        let path = M::to_path(base_dir, &id);
        let file = std::fs::File::open(path).unwrap();
        rmp_serde::from_read::<_, M>(file).unwrap()
    }
}
