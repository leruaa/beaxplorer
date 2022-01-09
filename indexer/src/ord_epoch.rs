use std::cmp::Ordering;

use db::models::EpochModel;

#[derive(Eq)]
pub struct OrderableEpoch<O: Ord + Eq> {
    pub epoch: i64,
    pub ordering: O,
}

impl<O: Ord> Ord for OrderableEpoch<O> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ordering.cmp(&other.ordering)
    }
}

impl<O: Ord> PartialOrd for OrderableEpoch<O> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<O: Ord> PartialEq for OrderableEpoch<O> {
    fn eq(&self, other: &Self) -> bool {
        self.epoch == other.epoch
    }
}

impl<O: Ord> From<(&EpochModel, O)> for OrderableEpoch<O> {
    fn from(from: (&EpochModel, O)) -> Self {
        OrderableEpoch {
            epoch: from.0.epoch,
            ordering: from.1,
        }
    }
}
