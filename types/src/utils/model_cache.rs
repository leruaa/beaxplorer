use std::{hash::Hash, num::NonZeroUsize};

use lru::LruCache;

use crate::{
    model::ModelWithId,
    path::FromPath,
    persistable::{Persistable, ResolvablePersistable},
};

#[derive(Debug)]
pub struct ModelCache<P>
where
    P: FromPath,
    P::Id: Hash + Eq,
{
    base_path: String,
    cache: LruCache<P::Id, ModelWithId<P::Id, P>>,
}

impl<P> ModelCache<P>
where
    P: FromPath,
    P::Id: Hash + Eq,
{
    pub fn new(base_path: String) -> Self {
        Self {
            base_path,
            cache: LruCache::new(NonZeroUsize::new(64).unwrap()),
        }
    }

    pub fn peek(&self, id: &P::Id) -> Option<&ModelWithId<P::Id, P>> {
        self.cache.peek(id)
    }

    pub fn contains(&self, id: &P::Id) -> bool {
        self.cache.contains(id)
    }
}

impl<P> ModelCache<P>
where
    P: FromPath,
    P::Id: Hash + Eq + Clone,
{
    pub fn put(&mut self, value: ModelWithId<P::Id, P>) {
        self.cache.put(value.id.clone(), value);
    }

    pub fn get_mut(&mut self, id: P::Id) -> Option<&mut ModelWithId<P::Id, P>> {
        let base_dir = self.base_path.clone();

        if !self.cache.contains(&id) {
            if let Ok(model) = P::from_path(&base_dir, &id) {

                self.cache.put(
                    id.clone(),
                    ModelWithId {
                        id: id.clone(),
                        model,
                    }
                );
            }
        }

        self.cache
            .get_mut(&id)
    }
    
    pub fn entry(&mut self, id: P::Id) -> Entry<'_, P> {
        if self.contains(&id) {
            Entry::Occupied(OccupiedEntry::new(self, id))
        }
        else {
            Entry::Vacant(VacantEntry::new(self, id))
        }
    }
}

impl<P> ModelCache<P>
where
    P: FromPath + Default,
    P::Id: Hash + Eq + Clone,
{
    pub fn get_or_default_mut(&mut self, id: P::Id) -> &mut ModelWithId<P::Id, P> {
        let base_path = self.base_path.clone();

        self.cache.get_or_insert_mut(id.clone(), || ModelWithId {
            id: id.clone(),
            model: P::from_path(&base_path, &id).unwrap_or_default(),
        })
    }
}

impl<P> ModelCache<P>
where
    P: FromPath + Persistable,
    P::Id: Hash + Eq + Clone,
{
    pub fn update_and_save<F>(&mut self, id: P::Id, f: F) -> Result<(), String>
    where
        F: FnOnce(&mut ModelWithId<P::Id, P>),
    {
        let base_path = self.base_path.clone();
        
        if let Some(model_with_id) = self.get_mut(id) {
            f(model_with_id);

            model_with_id.save(&base_path)?;
        }

        Ok(())
    }
}

pub enum Entry<'a, P>
where
    P: FromPath,
    P::Id: Hash + Eq + Clone
{
    Occupied(OccupiedEntry<'a, P>),
    Vacant(VacantEntry<'a, P>),
}

impl<'a, P> Entry<'a, P>
where
    P: FromPath,
    P::Id: Hash + Eq + Clone
{
    pub fn and_modify<F>(self, f: F) -> Self
    where F: FnOnce(&mut ModelWithId<P::Id, P>)
    {
        match self {
            Entry::Occupied(mut e) => {
                f(e.get_mut());
                Entry::Occupied(e)
            },
            Entry::Vacant(e) =>Entry::Vacant(e),
        }
    }

    pub fn or_insert(self, model: P) -> &'a mut ModelWithId<P::Id, P> {
        match self {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => e.insert(model),
        }
    }

    pub fn or_insert_with<F: FnOnce() -> P>(self, default: F) -> &'a mut ModelWithId<P::Id, P> {
        match self {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(default()),
        }
    }

    pub fn update_or_insert(self, model: P) -> &'a mut ModelWithId<P::Id, P> {
        match self {
            Entry::Occupied(e) => e.update(model),
            Entry::Vacant(e) => e.insert(model),
        }
    }
}

pub struct OccupiedEntry<'a, P>
where
    P: FromPath,
    P::Id: Hash + Eq + Clone
{
    cache: &'a mut ModelCache<P>,
    key: P::Id,
}


impl<'a, P> OccupiedEntry<'a, P>
where
    P: FromPath,
    P::Id: Hash + Eq + Clone
{
    pub fn new(
        cache: &'a mut ModelCache<P>,
        key: P::Id,
    ) -> Self {
        Self {
            cache,
            key,
        }
    }

    pub fn update(self, model: P) -> &'a mut ModelWithId<P::Id, P> {
        self.cache.put(ModelWithId { id: self.key.clone(), model });
        self.cache.get_mut(self.key).expect("Should not happen")
    }

    pub fn get_mut(&mut self) -> &mut ModelWithId<P::Id, P> {
        self.cache.get_mut(self.key.clone()).expect("Should not happen")
    }

    pub fn into_mut(self) -> &'a mut ModelWithId<P::Id, P> {
        self.cache.get_mut(self.key).expect("Should not happen")
    }
}

pub struct VacantEntry<'a, P>
where
    P: FromPath,
    P::Id: Hash + Eq
{
    cache: &'a mut ModelCache<P>,
    key: P::Id,
}

impl<'a, P> VacantEntry<'a, P>
where
    P: FromPath,
    P::Id: Hash + Eq +
{
    pub fn new(cache: &'a mut ModelCache<P>, key: P::Id) -> Self {
        Self {
            cache,
            key,
        }
    }
}

impl<'a, P> VacantEntry<'a, P>
where
    P: FromPath,
    P::Id: Hash + Eq + Clone
{
    pub fn insert(self, model: P) -> &'a mut ModelWithId<P::Id, P> {
        self.cache.put(ModelWithId { id: self.key.clone(), model });
        self.cache.get_mut(self.key).expect("Should not happen")
    }
}