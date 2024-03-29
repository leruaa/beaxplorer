use std::{fs, path::PathBuf};

use serde::de::DeserializeOwned;

pub trait Prefix {
    fn prefix() -> String;

    fn sortable_field_prefix(field_name: &str) -> String {
        format!("{}/s/{}", Self::prefix(), field_name)
    }
}

pub trait ToPath: Sized {
    type Id;

    fn to_path(base_dir: &str, id: &Self::Id) -> String;
}

pub trait Dirs: Prefix {
    fn dirs(base_dir: &str) -> Vec<PathBuf>;

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

    fn to_path(base_dir: &str, id: &T::Id) -> String {
        T::to_path(base_dir, id)
    }
}

impl<T> ToPath for Vec<T>
where
    T: ToPath,
{
    type Id = T::Id;

    fn to_path(base_dir: &str, id: &T::Id) -> String {
        T::to_path(base_dir, id)
    }
}

pub trait FromPath: ToPath + DeserializeOwned {
    fn from_path(base_dir: &str, id: &Self::Id) -> Result<Self, String> {
        let path = Self::to_path(base_dir, id);
        let file = std::fs::File::open(path.clone())
            .map_err(|err| format!("Can't open '{path}': {err}"))?;
        rmp_serde::from_read::<_, Self>(file)
            .map_err(|err| format!("Can't deserialize {path}: {err}"))
    }
}

impl<T: ToPath + DeserializeOwned> FromPath for T {}
