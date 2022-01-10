use types::views::EpochView;

pub trait Indexable {
    fn get_id(&self) -> u64;
}

impl Indexable for EpochView {
    fn get_id(&self) -> u64 {
        self.epoch
    }
}
