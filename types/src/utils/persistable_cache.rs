use std::{collections::HashSet, hash::Hash, num::NonZeroUsize};

use lru::LruCache;
use serde::Serialize;

use crate::{
    model::ModelWithId,
    path::{FromPath, ToPath},
    persistable::Persistable,
};

#[derive(Debug)]
pub struct PersistableCache<P>
where
    P: FromPath,
    P::Id: Hash + Eq,
{
    base_dir: String,
    cache: LruCache<P::Id, ModelWithId<P::Id, P::Model>>,
    dirty: HashSet<P::Id>,
}

impl<P> PersistableCache<P>
where
    P: FromPath,
    P::Id: Hash + Eq,
{
    pub fn new(base_dir: String) -> Self {
        Self {
            base_dir,
            cache: LruCache::new(NonZeroUsize::new(64).unwrap()),
            dirty: HashSet::new(),
        }
    }
}

impl<P> PersistableCache<P>
where
    P: FromPath,
    P::Id: Hash + Eq + Clone,
{
    pub fn get_mut(&mut self, id: P::Id) -> Result<&mut ModelWithId<P::Id, P::Model>, String> {
        let base_dir = self.base_dir.clone();

        if !self.cache.contains(&id) {
            let model = ModelWithId {
                id: id.clone(),
                model: P::from_path(&base_dir, &id)?,
            };
            self.cache.put(id.clone(), model);
        }

        self.dirty.insert(id.clone());

        self.cache
            .get_mut(&id)
            .ok_or(String::from("Should never happen"))
    }
}

impl<P> PersistableCache<P>
where
    P: FromPath,
    P::Id: Hash + Eq + Clone,
    P::Model: Default,
{
    pub fn get_or_default_mut(&mut self, id: P::Id) -> &mut ModelWithId<P::Id, P::Model> {
        let base_dir = self.base_dir.clone();

        self.dirty.insert(id.clone());

        self.cache.get_or_insert_mut(id.clone(), || ModelWithId {
            id: id.clone(),
            model: P::from_path(&base_dir, &id).unwrap_or_default(),
        })
    }
}

impl<P> PersistableCache<P>
where
    P: FromPath,
    P::Id: Hash + Eq,
    P::Model: Serialize + ToPath<Id = P::Id>,
{
    pub fn persist_dirty(&mut self) {
        let base_dir = self.base_dir.clone();

        self.dirty.iter().for_each(|s| {
            if let Some(persistable) = self.cache.peek(s) {
                persistable.persist(&base_dir)
            }
        });

        self.dirty.clear();
    }
}
