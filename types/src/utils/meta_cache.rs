use std::{
    collections::{hash_map::Entry, HashMap},
    ops::{Deref, DerefMut}, marker::PhantomData,
};

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
                } else {
                    None
                }
            }
        }
    }

    pub fn get_or<M>(&mut self, default: Meta) -> PersistableMeta<M>
    where
        M: Prefix,
    {
        let full_path = Meta::to_path::<M>(&self.base_path);

        let meta = self
            .cache
            .entry(full_path.clone())
            .or_insert(Meta::deserialize_from_file(&full_path).unwrap_or(default));

        PersistableMeta::new(self.base_path.clone(), meta)
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

pub struct PersistableMeta<'a, M: Prefix> {
    base_path: String,
    meta: &'a mut Meta,
    phantom: PhantomData<M>,
}

impl<'a, M: Prefix> PersistableMeta<'a, M> {
    pub fn new(base_path: String, meta: &'a mut Meta) -> Self {
        Self { base_path, meta, phantom: PhantomData::default() }
    }

    pub fn persist(&self) -> Result<(), String> {
        self.meta.serialize_to_file(&Meta::to_path::<M>(&self.base_path))
    }
}

impl<'a, M: Prefix> Deref for PersistableMeta<'a, M> {
    type Target = Meta;

    fn deref(&self) -> &Self::Target {
        self.meta
    }
}

impl<'a, M: Prefix> DerefMut for PersistableMeta<'a, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.meta
    }
}
