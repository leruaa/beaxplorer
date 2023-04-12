use std::{hash::Hash, num::NonZeroUsize};

use lru::LruCache;
use serde::Serialize;

use crate::{
    model::ModelWithId,
    path::{FromPath, ToPath},
    persistable::Persistable,
};

#[derive(Debug)]
pub struct ModelCache<P>
where
    P: FromPath,
    P::Id: Hash + Eq,
{
    base_dir: String,
    cache: LruCache<P::Id, ModelWithId<P::Id, P::Model>>,
}

impl<P> ModelCache<P>
where
    P: FromPath,
    P::Id: Hash + Eq,
{
    pub fn new(base_dir: String) -> Self {
        Self {
            base_dir,
            cache: LruCache::new(NonZeroUsize::new(64).unwrap()),
        }
    }

    pub fn peek(&self, id: &P::Id) -> Option<&ModelWithId<P::Id, P::Model>> {
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
    pub fn put(&mut self, value: ModelWithId<P::Id, P::Model>) {
        self.cache.put(value.id.clone(), value);
    }

    pub fn get_mut(&mut self, id: P::Id) -> Result<&mut ModelWithId<P::Id, P::Model>, String> {
        let base_dir = self.base_dir.clone();

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
    P: FromPath,
    P::Id: Hash + Eq + Clone,
    P::Model: Default,
{
    pub fn get_or_default_mut(&mut self, id: P::Id) -> &mut ModelWithId<P::Id, P::Model> {
        let base_dir = self.base_dir.clone();

        self.cache.get_or_insert_mut(id.clone(), || ModelWithId {
            id: id.clone(),
            model: P::from_path(&base_dir, &id).unwrap_or_default(),
        })
    }
}

impl<P> ModelCache<P>
where
    P: FromPath,
    P::Id: Hash + Eq + Clone,
    P::Model: Serialize + ToPath<Id = P::Id>,
{
    pub fn update_and_persist<F>(&mut self, id: P::Id, f: F) -> Result<(), String>
    where
        F: FnOnce(&mut ModelWithId<P::Id, P::Model>),
    {
        let base_dir = self.base_dir.clone();
        let model = self.get_mut(id)?;

        f(model);

        model.persist(&base_dir);

        Ok(())
    }
}
