use std::{fs::File, io::BufWriter};

use crate::types::meta::Meta;
use rmp_serde::Serializer;
use serde::Serialize;
use types::persisting_path::PersistingPathWithId;

pub trait Persistable: Send {
    fn persist(self, base_dir: &str) -> ();
}

impl<T> Persistable for T
where
    T: Meta,
{
    fn persist(self, base_dir: &str) -> () {
        let mut f = BufWriter::new(File::create(format!("{}/{}", base_dir, T::to_path())).unwrap());
        self.serialize(&mut Serializer::new(&mut f)).unwrap();
    }
}

impl<M> Persistable for (u64, M)
where
    M: Serialize + Send,
    (u64, M): PersistingPathWithId<u64>,
{
    fn persist(self, base_dir: &str) -> () {
        let mut f = BufWriter::new(
            File::create(format!("{}/{}", base_dir, Self::to_path(self.0))).unwrap(),
        );
        self.1.serialize(&mut Serializer::new(&mut f)).unwrap();
    }
}

impl<M> Persistable for Vec<(u64, M)>
where
    M: Serialize + Send,
    (u64, M): PersistingPathWithId<u64>,
{
    fn persist(self, base_dir: &str) -> () {
        for m in self {
            m.persist(base_dir);
        }
    }
}
