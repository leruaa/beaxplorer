use std::{fs::File, io::BufWriter};

use crate::{model::ModelWithId, path::ToPath};
use rmp_serde::Serializer;
use serde::Serialize;

pub trait Persistable {
    fn persist(self, base_dir: &str);
}

impl<M> Persistable for ModelWithId<M>
where
    M: Serialize,
    ModelWithId<M>: ToPath<u64>,
{
    fn persist(self, base_dir: &str) {
        let path = Self::to_path(base_dir, self.id);
        let mut f = BufWriter::new(File::create(path).unwrap());
        self.model.serialize(&mut Serializer::new(&mut f)).unwrap();
    }
}

impl<M> Persistable for Option<ModelWithId<M>>
where
    M: Serialize,
    ModelWithId<M>: ToPath<u64>,
{
    fn persist(self, base_dir: &str) {
        if let Some(model_with_id) = self {
            model_with_id.persist(base_dir)
        }
    }
}

impl<M> Persistable for Vec<ModelWithId<M>>
where
    M: Serialize,
    ModelWithId<M>: ToPath<u64>,
{
    fn persist(self, base_dir: &str) {
        for m in self {
            m.persist(base_dir);
        }
    }
}

pub trait PersistableIterator {
    fn persist(self, base_dir: &str);
}

impl<I> PersistableIterator for I
where
    I: Iterator,
    I::Item: Persistable,
{
    fn persist(self, base_dir: &str) {
        for m in self {
            m.persist(base_dir)
        }
    }
}
