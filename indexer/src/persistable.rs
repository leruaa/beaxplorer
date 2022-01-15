use std::{fs::File, io::BufWriter};

use rmp_serde::Serializer;
use serde::Serialize;
use types::{
    meta::EpochsMeta,
    views::{BlockView, EpochView},
};

pub trait Persistable<I>: Send {
    fn persist(self, base_dir: &str) -> ();
}

impl Persistable<EpochView> for EpochView {
    fn persist(self, base_dir: &str) -> () {
        let mut f = BufWriter::new(
            File::create(format!("{}/epochs/{}.msg", base_dir, self.epoch)).unwrap(),
        );
        self.serialize(&mut Serializer::new(&mut f)).unwrap();
    }
}

impl Persistable<BlockView> for BlockView {
    fn persist(self, base_dir: &str) -> () {
        let mut f =
            BufWriter::new(File::create(format!("{}/blocks/{}.msg", base_dir, self.slot)).unwrap());
        self.serialize(&mut Serializer::new(&mut f)).unwrap();
    }
}

impl Persistable<EpochsMeta> for EpochsMeta {
    fn persist(self, base_dir: &str) -> () {
        let mut f = BufWriter::new(File::create(format!("{}/epochs/meta.msg", base_dir)).unwrap());
        self.serialize(&mut Serializer::new(&mut f)).unwrap();
    }
}
