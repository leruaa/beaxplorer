use std::collections::{HashMap, hash_map::Entry};

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

    pub fn get_mut<M>(&mut self) -> Option<&mut Meta>
    where
        M: Prefix,
    {
        let full_path = Meta::to_path::<M>(&self.base_path);

        match self.cache.entry(full_path.clone()) {
            Entry::Occupied(e) => Some(e.into_mut()),
            Entry::Vacant(e) => {
                if let Ok(meta) = Meta::deserialize_from_file(&full_path) {
                    Some(e.insert(meta))
                }
                else {
                    None
                }
            },
        }
    }

    pub fn get_mut_or<M>(&mut self, default: Meta) -> &mut Meta
    where
        M: Prefix,
    {
        let full_path = Meta::to_path::<M>(&self.base_path);

        self.cache
            .entry(full_path.clone())
            .or_insert(Meta::deserialize_from_file(&full_path).unwrap_or(default))
    }

    pub fn count<M>(&self) -> usize
    where
        M: Prefix,
    {
        let full_path = Meta::to_path::<M>(&self.base_path);
        self.cache.get(&full_path).map_or(0, |m| m.count)
    }

    pub fn get<M>(&mut self) -> Option<&Meta>
    where
        M: Prefix,
    {
        self.get_mut::<M>().map(|m| &*m)
    }

    pub fn get_or<M>(&mut self, default: Meta) -> &Meta
    where
        M: Prefix,
    {
        self.get_mut_or::<M>(default)
    }

    pub fn update_and_save<M, F>(&mut self, f: F) -> Result<(), String>
    where
        M: Prefix,
        F: FnOnce(&mut Meta),
    {
        let base_path = self.base_path.clone();
        let meta = self.get_mut::<M>().ok_or("Unable to retrieve meta")?;

        f(meta);

        meta.serialize_to_file(&Meta::to_path::<M>(&base_path))?;

        Ok(())
    }
}
