use serde::Serialize;
use types::{block::BlocksMeta, epoch::EpochsMeta, validator::ValidatorsMeta};

use crate::persisting_path::PersistingPath;

pub trait Meta: PersistingPath + Serialize + Send {}

impl Meta for EpochsMeta {}

impl Meta for BlocksMeta {}

impl Meta for ValidatorsMeta {}
