use crate::{
    attestation::AttestationsModelWithId,
    block::{BlockExtendedModelWithId, BlockModelWithId},
    committee::CommitteesModelWithId,
    epoch::{EpochExtendedModelWithId, EpochModelWithId},
    validator::ValidatorModelWithId,
    vote::VotesModelWithId,
};

pub trait AsPath {
    fn as_path(&self, base: &str) -> String;
}

pub trait ToPath {
    fn to_path(base: &str, id: u64) -> String;
}

impl ToPath for EpochModelWithId {
    fn to_path(base: &str, id: u64) -> String {
        format!("{}/epochs/{}.msg", base, id)
    }
}

impl ToPath for EpochExtendedModelWithId {
    fn to_path(base: &str, id: u64) -> String {
        format!("{}/epochs/e/{}.msg", base, id)
    }
}

impl ToPath for BlockModelWithId {
    fn to_path(base: &str, id: u64) -> String {
        format!("{}/blocks/{}.msg", base, id)
    }
}

impl ToPath for BlockExtendedModelWithId {
    fn to_path(base: &str, id: u64) -> String {
        format!("{}/blocks/e/{}.msg", base, id)
    }
}

impl ToPath for CommitteesModelWithId {
    fn to_path(base: &str, id: u64) -> String {
        format!("{}/blocks/c/{}.msg", base, id)
    }
}

impl ToPath for VotesModelWithId {
    fn to_path(base: &str, id: u64) -> String {
        format!("{}/blocks/v/{}.msg", base, id)
    }
}

impl ToPath for AttestationsModelWithId {
    fn to_path(base: &str, id: u64) -> String {
        format!("{}/blocks/a/{}.msg", base, id)
    }
}

impl ToPath for ValidatorModelWithId {
    fn to_path(base: &str, id: u64) -> String {
        format!("{}/validators/{}.msg", base, id)
    }
}
