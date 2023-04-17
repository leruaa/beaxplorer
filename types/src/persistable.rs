use rmp_serde::Serializer;
use serde::Serialize;
use std::{fs::File, io::BufWriter};

pub trait SelfPersistable: Serialize {
    fn persist(&self, full_path: &str) -> Result<(), String> {
        let file = File::create(full_path)
            .map_err(|err| format!("Can't create file '{full_path}': {err}"))?;
        let mut f = BufWriter::new(file);
        self.serialize(&mut Serializer::new(&mut f))
            .map_err(|err| format!("Can't serialize {full_path}: {err}"))
    }
}

pub trait Persistable {
    fn persist(&self, full_path: &str) -> Result<(), String>;
}

impl<T: SelfPersistable> Persistable for T {
    fn persist(&self, full_path: &str) -> Result<(), String> {
        T::persist(&self, full_path)
    }
}

impl<T> Persistable for Option<T>
where
    T: Persistable,
{
    fn persist(&self, full_path: &str) -> Result<(), String> {
        if let Some(p) = self {
            p.persist(full_path)
        } else {
            Ok(())
        }
    }
}

impl<T> Persistable for Vec<T>
where
    T: Persistable,
{
    fn persist(&self, full_path: &str) -> Result<(), String> {
        self.iter().try_for_each(|p| p.persist(full_path))
    }
}

pub trait ResolvablePersistable {
    fn save(&self, base_path: &str) -> Result<(), String>;
}

impl<T> ResolvablePersistable for Option<T>
where
    T: ResolvablePersistable,
{
    fn save(&self, base_path: &str) -> Result<(), String> {
        if let Some(p) = self {
            p.save(base_path)
        } else {
            Ok(())
        }
    }
}

impl<T> ResolvablePersistable for Vec<T>
where
    T: ResolvablePersistable,
{
    fn save(&self, base_path: &str) -> Result<(), String> {
        self.iter().try_for_each(|p| p.save(base_path))
    }
}
