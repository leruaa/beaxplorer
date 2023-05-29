use std::{
    collections::{hash_map::Entry as HashMapEntry, HashMap},
    marker::PhantomData,
};

use crate::{meta::Meta, path::Prefix, persistable::MsgPackDeserializable};

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

    pub fn insert<M>(&mut self, meta: Meta)
    where
        M: Prefix,
    {
        let full_path = Meta::to_path::<M>(&self.base_path);

        self.cache.insert(full_path, meta);
    }

    pub fn get_mut<M>(&mut self) -> Option<&mut Meta>
    where
        M: Prefix,
    {
        let full_path = Meta::to_path::<M>(&self.base_path);

        match self.cache.entry(full_path.clone()) {
            HashMapEntry::Occupied(e) => Some(e.into_mut()),
            HashMapEntry::Vacant(e) => {
                if let Ok(meta) = Meta::deserialize_from_file(&full_path) {
                    Some(e.insert(meta))
                } else {
                    None
                }
            }
        }
    }

    pub fn get_or<M>(&mut self, default: Meta) -> &Meta
    where
        M: Prefix,
    {
        let full_path = Meta::to_path::<M>(&self.base_path);

        let meta = self
            .cache
            .entry(full_path.clone())
            .or_insert(Meta::deserialize_from_file(&full_path).unwrap_or(default));

        meta
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

    pub fn entry<M>(&mut self) -> Entry<'_, M>
    where
        M: Prefix,
    {
        let full_path = Meta::to_path::<M>(&self.base_path);

        if self.cache.contains_key(&full_path) {
            Entry::Occupied(OccupiedEntry::new(self))
        } else {
            Entry::Vacant(VacantEntry::new(self))
        }
    }
}

pub enum Entry<'a, M>
where
    M: Prefix,
{
    Occupied(OccupiedEntry<'a, M>),
    Vacant(VacantEntry<'a, M>),
}

impl<'a, M> Entry<'a, M>
where
    M: Prefix,
{
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut Meta),
    {
        match self {
            Entry::Occupied(mut e) => {
                f(e.get_mut());
                Entry::Occupied(e)
            }
            Entry::Vacant(e) => Entry::Vacant(e),
        }
    }

    pub fn or_insert(self, meta: Meta) -> &'a mut Meta {
        match self {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => e.insert(meta),
        }
    }

    pub fn or_insert_with<F: FnOnce() -> Meta>(self, default: F) -> &'a mut Meta {
        match self {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(default()),
        }
    }

    pub fn increment_by(self, count: usize) -> &'a mut Meta {
        let meta = match self {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(Meta::default()),
        };

        meta.count += count;

        meta
    }

    pub fn increment(self) -> &'a mut Meta {
        self.increment_by(1)
    }

    pub fn update_count(self, value: usize) -> &'a mut Meta {
        let meta = match self {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(Meta::default()),
        };

        meta.count = value;

        meta
    }
}

pub struct OccupiedEntry<'a, M>
where
    M: Prefix,
{
    cache: &'a mut MetaCache,
    phantom: PhantomData<M>,
}

impl<'a, M> OccupiedEntry<'a, M>
where
    M: Prefix,
{
    pub fn new(cache: &'a mut MetaCache) -> Self {
        Self {
            cache,
            phantom: PhantomData::default(),
        }
    }

    pub fn get_mut(&mut self) -> &mut Meta {
        self.cache.get_mut::<M>().expect("Should not happen")
    }

    pub fn into_mut(self) -> &'a mut Meta {
        self.cache.get_mut::<M>().expect("Should not happen")
    }
}

pub struct VacantEntry<'a, M>
where
    M: Prefix,
{
    cache: &'a mut MetaCache,
    phantom: PhantomData<M>,
}

impl<'a, M> VacantEntry<'a, M>
where
    M: Prefix,
{
    pub fn new(cache: &'a mut MetaCache) -> Self {
        Self {
            cache,
            phantom: PhantomData::default(),
        }
    }

    pub fn insert(self, meta: Meta) -> &'a mut Meta {
        self.cache.insert::<M>(meta);
        self.cache.get_mut::<M>().expect("Should not happen")
    }
}
