use rmp_serde::Serializer;
use serde::{de::DeserializeOwned, Serialize};
use std::{fs::File, io::BufWriter};

pub trait MsgPackSerializable: Serialize {
    fn serialize_to_file(&self, full_path: &str) -> Result<(), String> {
        let file = File::create(full_path)
            .map_err(|err| format!("Can't create file '{full_path}': {err}"))?;
        let mut f = BufWriter::new(file);
        self.serialize(&mut Serializer::new(&mut f))
            .map_err(|err| format!("Can't serialize {full_path}: {err}"))
    }
}

impl<T> MsgPackSerializable for Vec<T> where T: MsgPackSerializable {}

pub trait MsgPackDeserializable: DeserializeOwned {
    fn deserialize_from_file(full_path: &str) -> Result<Self, String> {
        let file = std::fs::File::open(full_path)
            .map_err(|err| format!("Can't open '{full_path}': {err}"))?;
        rmp_serde::from_read::<_, Self>(file)
            .map_err(|err| format!("Can't deserialize {full_path}: {err}"))
    }
}

pub trait Persistable {
    fn persist(&self, full_path: &str) -> Result<(), String>;
}

impl<T: MsgPackSerializable> Persistable for T {
    fn persist(&self, full_path: &str) -> Result<(), String> {
        T::serialize_to_file(self, full_path)
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
