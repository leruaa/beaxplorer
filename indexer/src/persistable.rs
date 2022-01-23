use std::{fs::File, io::BufWriter};

use crate::types::model::Model;
use rmp_serde::Serializer;

pub trait Persistable: Send {
    fn persist(self, base_dir: &str) -> ();
}

impl<M: Model> Persistable for M {
    fn persist(self, base_dir: &str) -> () {
        let mut f = BufWriter::new(
            File::create(format!(
                "{}/{}/{}.msg",
                base_dir,
                M::get_path(),
                self.get_id()
            ))
            .unwrap(),
        );
        self.serialize(&mut Serializer::new(&mut f)).unwrap();
    }
}

impl<M: Model> Persistable for Vec<M> {
    fn persist(self, base_dir: &str) -> () {
        for m in self {
            m.persist(base_dir);
        }
    }
}
