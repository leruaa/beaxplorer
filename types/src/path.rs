use std::{fs, path::PathBuf};

use serde::de::DeserializeOwned;

pub trait ToPath<Id>: Sized {
    fn prefix() -> String;

    fn to_path(base_dir: &str, id: &Id) -> String;

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

impl<T, Id> ToPath<Id> for Option<T>
where
    T: ToPath<Id>,
{
    fn prefix() -> String {
        T::prefix()
    }

    fn to_path(base_dir: &str, id: &Id) -> String {
        T::to_path(base_dir, id)
    }

    fn dirs(base_dir: &str) -> Vec<PathBuf> {
        T::dirs(base_dir)
    }
}

impl<T, Id> ToPath<Id> for Vec<T>
where
    T: ToPath<Id>,
{
    fn prefix() -> String {
        T::prefix()
    }

    fn to_path(base_dir: &str, id: &Id) -> String {
        T::to_path(base_dir, id)
    }

    fn dirs(base_dir: &str) -> Vec<PathBuf> {
        T::dirs(base_dir)
    }
}

pub trait FromPath<Id, M> {
    fn from_path(base_dir: &str, id: &Id) -> M;
}

impl<Id, M> FromPath<Id, M> for M
where
    M: ToPath<Id> + DeserializeOwned,
{
    fn from_path(base_dir: &str, id: &Id) -> Self {
        let path = Self::to_path(base_dir, id);
        let file = std::fs::File::open(path).unwrap();
        rmp_serde::from_read::<_, M>(file).unwrap()
    }
}
