use serde::Serialize;
use types::{
    block::BlocksMeta, epoch::EpochsMeta, persisting_path::PersistingPath,
    validator::ValidatorsMeta,
};

pub trait Meta: PersistingPath + Serialize + Send {}

impl Meta for EpochsMeta {}

impl Meta for BlocksMeta {}

impl Meta for ValidatorsMeta {}
