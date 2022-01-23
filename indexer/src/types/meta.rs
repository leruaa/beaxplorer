use types::{block::BlocksMeta, epoch::EpochsMeta, validator::ValidatorsMeta};

pub trait Meta: Send {
    fn get_path() -> &'static str;
}

impl Meta for EpochsMeta {
    fn get_path() -> &'static str {
        "epochs"
    }
}

impl Meta for BlocksMeta {
    fn get_path() -> &'static str {
        "blocks"
    }
}

impl Meta for ValidatorsMeta {
    fn get_path() -> &'static str {
        "validators"
    }
}
