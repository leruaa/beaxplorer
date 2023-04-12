use std::{collections::HashSet, hash::Hash};

use serde::Serialize;

use crate::{
    model::ModelWithId,
    path::{FromPath, ToPath},
    persistable::Persistable,
};

use super::ModelCache;

pub struct PersistableCache<P>
where
    P: FromPath,
    P::Id: Hash + Eq,
{
    base_dir: String,
    cache: ModelCache<P>,
    dirty: HashSet<P::Id>,
}

impl<P> PersistableCache<P>
where
    P: FromPath,
    P::Id: Hash + Eq + Clone,
{
    pub fn new(base_dir: String) -> Self {
        Self {
            base_dir: base_dir.clone(),
            cache: ModelCache::new(base_dir),
            dirty: HashSet::new(),
        }
    }

    pub fn put(&mut self, value: ModelWithId<P::Id, P::Model>) {
        self.cache.put(value);
    }

    pub fn contains(&self, id: P::Id) -> bool {
        self.cache.contains(id)
    }
}

impl<P> PersistableCache<P>
where
    P: FromPath,
    P::Id: Hash + Eq + Clone,
{
    pub fn get_mut(&mut self, id: P::Id) -> Result<&mut ModelWithId<P::Id, P::Model>, String> {
        self.dirty.insert(id.clone());
        self.cache.get_mut(id)
    }

    pub fn dirty_iter(&self) -> impl Iterator<Item = &ModelWithId<P::Id, P::Model>>
    {
        self.dirty.iter().filter_map(move |id| self.cache.peek(id))
    }
}

impl<P> PersistableCache<P>
where
    P: FromPath,
    P::Id: Hash + Eq + Clone,
    P::Model: Default,
{
    pub fn get_or_default_mut(&mut self, id: P::Id) -> &mut ModelWithId<P::Id, P::Model> {
        self.dirty.insert(id.clone());
        self.cache.get_or_default_mut(id)
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
