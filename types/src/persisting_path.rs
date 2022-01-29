use crate::{
    block::{BlockExtendedModelWithId, BlockModelWithId, BlocksMeta},
    epoch::{EpochExtendedModelWithId, EpochModelWithId, EpochsMeta},
    validator::{ValidatorModelWithId, ValidatorsMeta},
};

pub trait PersistingPath {
    fn to_path() -> String;
}

pub trait PersistingPathWithId<Id: ToString> {
    fn to_path(id: Id) -> String;
}

impl PersistingPathWithId<u64> for EpochModelWithId {
    fn to_path(id: u64) -> String {
        format!("epochs/{}.msg", id)
    }
}

impl PersistingPathWithId<u64> for EpochExtendedModelWithId {
    fn to_path(id: u64) -> String {
        format!("epochs/e/{}.msg", id)
    }
}

impl PersistingPath for EpochsMeta {
    fn to_path() -> String {
        "epochs/meta.msg".to_string()
    }
}

impl PersistingPathWithId<u64> for BlockModelWithId {
    fn to_path(id: u64) -> String {
        format!("blocks/{}.msg", id)
    }
}

impl PersistingPathWithId<u64> for BlockExtendedModelWithId {
    fn to_path(id: u64) -> String {
        format!("blocks/e/{}.msg", id)
    }
}

impl PersistingPath for BlocksMeta {
    fn to_path() -> String {
        "blocks/meta.msg".to_string()
    }
}

impl PersistingPathWithId<u64> for ValidatorModelWithId {
    fn to_path(id: u64) -> String {
        format!("validators/{}.msg", id)
    }
}

impl PersistingPath for ValidatorsMeta {
    fn to_path() -> String {
        "validators/meta.msg".to_string()
    }
}
