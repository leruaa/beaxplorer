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

    pub fn contains(&self, id: P::Id) -> bool {
        self.cache.contains(&id)
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

    pub fn get_mut(&mut self, id: P::Id) -> Result<&mut ModelWithId<P::Id, P>, String> {
        let base_dir = self.base_path.clone();

        if !self.cache.contains(&id) {
            let model = ModelWithId {
                id: id.clone(),
                model: P::from_path(&base_dir, &id)?,
            };
            self.cache.put(id.clone(), model);
        }

        self.cache
            .get_mut(&id)
            .ok_or(String::from("Should never happen"))
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
        let model_with_id = self.get_mut(id)?;

        f(model_with_id);

        model_with_id.save(&base_path)?;

        Ok(())
    }
}
