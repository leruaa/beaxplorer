use std::{fs::File, io::BufWriter};

use rmp_serde::Serializer;
use serde::Serialize;
use types::{
    block::BlocksMeta, epoch::EpochsMeta, meta::Meta, model::ModelWithId, path::AsPath,
    validator::ValidatorsMeta,
};

pub trait Persistable: Send {
    fn persist(self, base_dir: &str);
}

impl Persistable for EpochsMeta {
    fn persist(self, base_dir: &str) {
        let mut f = BufWriter::new(File::create(Self::to_path(base_dir)).unwrap());
        self.serialize(&mut Serializer::new(&mut f)).unwrap();
    }
}

impl Persistable for BlocksMeta {
    fn persist(self, base_dir: &str) {
        let mut f = BufWriter::new(File::create(Self::to_path(base_dir)).unwrap());
        self.serialize(&mut Serializer::new(&mut f)).unwrap();
    }
}

impl Persistable for ValidatorsMeta {
    fn persist(self, base_dir: &str) {
        let mut f = BufWriter::new(File::create(Self::to_path(base_dir)).unwrap());
        self.serialize(&mut Serializer::new(&mut f)).unwrap();
    }
}

impl<M> Persistable for ModelWithId<M>
where
    M: Serialize + Send,
    ModelWithId<M>: AsPath,
{
    fn persist(self, base_dir: &str) {
        let mut f = BufWriter::new(File::create(self.as_path(base_dir)).unwrap());
        self.model.serialize(&mut Serializer::new(&mut f)).unwrap();
    }
}

impl<M> Persistable for Option<ModelWithId<M>>
where
    M: Serialize + Send,
    ModelWithId<M>: AsPath,
{
    fn persist(self, base_dir: &str) {
        if let Some(model_with_id) = self {
            let mut f = BufWriter::new(File::create(model_with_id.as_path(base_dir)).unwrap());
            model_with_id
                .model
                .serialize(&mut Serializer::new(&mut f))
                .unwrap();
        }
    }
}

impl<M> Persistable for Vec<ModelWithId<M>>
where
    M: Serialize + Send,
    ModelWithId<M>: AsPath,
{
    fn persist(self, base_dir: &str) {
        for m in self {
            m.persist(base_dir);
        }
    }
}
