use std::{fs::File, io::BufWriter};

use rmp_serde::Serializer;
use serde::Serialize;
use types::{
    meta::{BlocksMeta, EpochsMeta, ValidatorsMeta},
    views::{BlockView, EpochView, ValidatorView},
};

pub trait Persistable: Send {
    fn persist(self, base_dir: &str) -> ();
}

impl Persistable for EpochView {
    fn persist(self, base_dir: &str) -> () {
        let mut f = BufWriter::new(
            File::create(format!("{}/epochs/{}.msg", base_dir, self.epoch)).unwrap(),
        );
        self.serialize(&mut Serializer::new(&mut f)).unwrap();
    }
}

impl Persistable for BlockView {
    fn persist(self, base_dir: &str) -> () {
        let mut f =
            BufWriter::new(File::create(format!("{}/blocks/{}.msg", base_dir, self.slot)).unwrap());
        self.serialize(&mut Serializer::new(&mut f)).unwrap();
    }
}

impl Persistable for Vec<ValidatorView> {
    fn persist(self, base_dir: &str) -> () {
        for v in self {
            let mut f = BufWriter::new(
                File::create(format!("{}/validators/{}.msg", base_dir, v.validator_index)).unwrap(),
            );
            v.serialize(&mut Serializer::new(&mut f)).unwrap();
        }
    }
}

impl Persistable for EpochsMeta {
    fn persist(self, base_dir: &str) -> () {
        let mut f = BufWriter::new(File::create(format!("{}/epochs/meta.msg", base_dir)).unwrap());
        self.serialize(&mut Serializer::new(&mut f)).unwrap();
    }
}

impl Persistable for BlocksMeta {
    fn persist(self, base_dir: &str) -> () {
        let mut f = BufWriter::new(File::create(format!("{}/blocks/meta.msg", base_dir)).unwrap());
        self.serialize(&mut Serializer::new(&mut f)).unwrap();
    }
}

impl Persistable for ValidatorsMeta {
    fn persist(self, base_dir: &str) -> () {
        let mut f =
            BufWriter::new(File::create(format!("{}/validators/meta.msg", base_dir)).unwrap());
        self.serialize(&mut Serializer::new(&mut f)).unwrap();
    }
}
