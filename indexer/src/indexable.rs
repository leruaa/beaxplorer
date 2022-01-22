use types::epoch::EpochModel;

pub trait Indexable: Send {
    fn get_id(&self) -> u64;
}

impl Indexable for EpochModel {
    fn get_id(&self) -> u64 {
        self.epoch
    }
}
