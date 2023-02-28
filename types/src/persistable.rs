use std::{fs::File, io::BufWriter};

use crate::{model::ModelWithId, path::ToPath};
use rmp_serde::Serializer;
use serde::Serialize;

pub trait Persistable {
    fn persist(&self, base_dir: &str);
}

impl<M> Persistable for ModelWithId<M>
where
    M: Serialize + ToPath<u64>,
{
    fn persist(&self, base_dir: &str) {
        let path = M::to_path(base_dir, self.id);
        let mut f = BufWriter::new(File::create(path).unwrap());
        self.model.serialize(&mut Serializer::new(&mut f)).unwrap();
    }
}

impl<T> Persistable for T
where
    T: Serialize + ToPath<()>,
{
    fn persist(&self, base_dir: &str) {
        let path = T::to_path(base_dir, ());
        let mut f = BufWriter::new(File::create(path).unwrap());
        self.serialize(&mut Serializer::new(&mut f)).unwrap();
    }
}
