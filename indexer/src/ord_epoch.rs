use std::cmp::Ordering;

#[derive(Eq)]
pub struct OrderableEpoch<O: Ord + Eq> {
    pub epoch: u64,
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

impl<O: Ord> From<(u64, O)> for OrderableEpoch<O> {
    fn from(from: (u64, O)) -> Self {
        OrderableEpoch {
            epoch: from.0,
            ordering: from.1,
        }
    }
}
