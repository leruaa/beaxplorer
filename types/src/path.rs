use std::{fs, path::PathBuf};

use serde::de::DeserializeOwned;

pub trait ToPath: Sized {
    type Id;

    fn prefix() -> String;

    fn to_path(base_dir: &str, id: &Self::Id) -> String;

    fn dirs(base_dir: &str) -> Vec<PathBuf>;

    fn sortable_field_prefix(field_name: &str) -> String {
        format!("{}/s/{}", Self::prefix(), field_name)
    }

    fn create_dirs(base_dir: &str) -> Result<(), String> {
        Self::dirs(base_dir).into_iter().try_for_each(|d| {
            fs::create_dir_all(&d).map_err(|err| format!("Failed to create '{d:?}': {err}"))
        })
    }

    fn remove_dirs(base_dir: &str) -> Result<(), String> {
        Self::dirs(base_dir).into_iter().try_for_each(|d| {
            if d.exists() {
                fs::remove_dir_all(&d).map_err(|err| format!("Failed to remove '{d:?}': {err}"))
            } else {
                Ok(())
            }
        })
    }
}

impl<T> ToPath for Option<T>
where
    T: ToPath,
{
    type Id = T::Id;

    fn prefix() -> String {
        T::prefix()
    }

    fn to_path(base_dir: &str, id: &T::Id) -> String {
        T::to_path(base_dir, id)
    }

    fn dirs(base_dir: &str) -> Vec<PathBuf> {
        T::dirs(base_dir)
    }
}

impl<T> ToPath for Vec<T>
where
    T: ToPath,
{
    type Id = T::Id;

    fn prefix() -> String {
        T::prefix()
    }

    fn to_path(base_dir: &str, id: &T::Id) -> String {
        T::to_path(base_dir, id)
    }

    fn dirs(base_dir: &str) -> Vec<PathBuf> {
        T::dirs(base_dir)
    }
}

pub trait FromPath {
    type Id;
    type Model;

    fn from_path(base_dir: &str, id: &Self::Id) -> Result<Self::Model, String>;
}

impl<M> FromPath for M
where
    M: ToPath + DeserializeOwned,
{
    type Id = M::Id;
    type Model = M;

    fn from_path(base_dir: &str, id: &Self::Id) -> Result<Self, String> {
        let path = Self::to_path(base_dir, id);
        let file = std::fs::File::open(path.clone())
            .map_err(|err| format!("Can't open '{path}': {err}"))?;
        rmp_serde::from_read::<_, M>(file).map_err(|err| format!("Can't deserialize {path}: {err}"))
    }
}
