use types::{
    block::{BlockModelWithId, BlocksMeta},
    epoch::{EpochExtendedModelWithId, EpochModelWithId, EpochsMeta},
    validator::{ValidatorModelWithId, ValidatorsMeta},
};

pub trait PersistingPath {
    fn to_path(&self) -> String;
}

impl PersistingPath for EpochModelWithId {
    fn to_path(&self) -> String {
        format!("epochs/{}.msg", self.0)
    }
}

impl PersistingPath for EpochExtendedModelWithId {
    fn to_path(&self) -> String {
        format!("epochs/e/{}.msg", self.0)
    }
}

impl PersistingPath for EpochsMeta {
    fn to_path(&self) -> String {
        "epochs/meta.msg".to_string()
    }
}

impl PersistingPath for BlockModelWithId {
    fn to_path(&self) -> String {
        format!("blocks/{}.msg", self.0)
    }
}

impl PersistingPath for BlocksMeta {
    fn to_path(&self) -> String {
        "blocks/meta.msg".to_string()
    }
}

impl PersistingPath for ValidatorModelWithId {
    fn to_path(&self) -> String {
        format!("validators/{}.msg", self.0)
    }
}

impl PersistingPath for ValidatorsMeta {
    fn to_path(&self) -> String {
        "validators/meta.msg".to_string()
    }
}
