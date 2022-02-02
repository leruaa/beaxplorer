use crate::{
    block::{BlockExtendedModelWithId, BlockModelWithId, BlocksMeta},
    committee::CommitteesModelWithId,
    epoch::{EpochExtendedModelWithId, EpochModelWithId, EpochsMeta},
    validator::{ValidatorModelWithId, ValidatorsMeta},
};

pub trait PersistingPath {
    fn to_path(base: &str) -> String;
}

pub trait PersistingPathWithId<Id: ToString> {
    fn to_path(base: &str, id: Id) -> String;
}

impl PersistingPathWithId<u64> for EpochModelWithId {
    fn to_path(base: &str, id: u64) -> String {
        format!("{}/epochs/{}.msg", base, id)
    }
}

impl PersistingPathWithId<u64> for EpochExtendedModelWithId {
    fn to_path(base: &str, id: u64) -> String {
        format!("{}/epochs/e/{}.msg", base, id)
    }
}

impl PersistingPath for EpochsMeta {
    fn to_path(base: &str) -> String {
        format!("{}/epochs/meta.msg", base)
    }
}

impl PersistingPathWithId<u64> for BlockModelWithId {
    fn to_path(base: &str, id: u64) -> String {
        format!("{}/blocks/{}.msg", base, id)
    }
}

impl PersistingPathWithId<u64> for BlockExtendedModelWithId {
    fn to_path(base: &str, id: u64) -> String {
        format!("{}/blocks/e/{}.msg", base, id)
    }
}

impl PersistingPathWithId<u64> for CommitteesModelWithId {
    fn to_path(base: &str, id: u64) -> String {
        format!("{}/blocks/c/{}.msg", base, id)
    }
}

impl PersistingPath for BlocksMeta {
    fn to_path(base: &str) -> String {
        format!("{}/blocks/meta.msg", base)
    }
}

impl PersistingPathWithId<u64> for ValidatorModelWithId {
    fn to_path(base: &str, id: u64) -> String {
        format!("{}/validators/{}.msg", base, id)
    }
}

impl PersistingPath for ValidatorsMeta {
    fn to_path(base: &str) -> String {
        format!("{}/validators/meta.msg", base)
    }
}
