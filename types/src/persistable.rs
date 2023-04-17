use crate::{model::ModelWithId, path::ToPath};
use rmp_serde::Serializer;
use serde::Serialize;
use std::{fs::File, io::BufWriter};

pub trait Persistable {
    fn persist(&self, base_dir: &str);
}

impl<Id, M> Persistable for ModelWithId<Id, M>
where
    M: Serialize + ToPath<Id = Id>,
{
    fn persist(&self, base_dir: &str) {
        let path = M::to_path(base_dir, &self.id);
        let mut f = BufWriter::new(File::create(path).unwrap());
        self.model.serialize(&mut Serializer::new(&mut f)).unwrap();
    }
}

impl<T> Persistable for Option<T>
where
    T: Persistable,
{
    fn persist(&self, base_dir: &str) {
        if let Some(persistable) = self {
            persistable.persist(base_dir)
        }
    }
}
