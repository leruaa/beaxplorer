use std::collections::HashMap;

use crate::{
    meta::Meta,
    path::Prefix,
    persistable::{MsgPackDeserializable, MsgPackSerializable},
};

#[derive(Debug)]
pub struct MetaCache {
    base_path: String,
    cache: HashMap<String, Meta>,
}

impl MetaCache {
    pub fn new(base_path: String) -> Self {
        Self {
            base_path,
            cache: HashMap::new(),
        }
    }

    pub fn get_mut<M>(&mut self) -> Result<&mut Meta, String>
    where
        M: Prefix,
    {
        let full_path = Meta::to_path::<M>(&self.base_path);
        if !self.cache.contains_key(&full_path) {
            self.cache.insert(
                full_path.clone(),
                Meta::deserialize_from_file(&full_path).unwrap_or_default(),
            );
        }

        self.cache
            .get_mut(&full_path)
            .ok_or(String::from("Should not happen"))
    }

    pub fn count<M>(&self) -> usize
    where
        M: Prefix,
    {
        let full_path = Meta::to_path::<M>(&self.base_path);
        self.cache.get(&full_path).map_or(0, |m| m.count)
    }

    pub fn get<M>(&mut self) -> Result<&Meta, String>
    where
        M: Prefix,
    {
        self.get_mut::<M>().map(|m| &*m)
    }

    pub fn update_and_save<M, F>(&mut self, f: F) -> Result<(), String>
    where
        M: Prefix,
        F: FnOnce(&mut Meta),
    {
        let base_path = self.base_path.clone();
        let meta = self.get_mut::<M>()?;

        f(meta);

        meta.serialize_to_file(&Meta::to_path::<M>(&base_path))?;

        Ok(())
    }
}
