use std::cmp::Ordering;

#[derive(Eq)]
pub struct Orderable<O: Ord> {
    pub id: u64,
    pub ordering: O,
}

impl<O: Ord> Ord for Orderable<O> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.ordering.cmp(&other.ordering) {
            Ordering::Equal => self.id.cmp(&other.id),
            o => o,
        }
    }
}

impl<O: Ord> PartialOrd for Orderable<O> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<O: Ord> PartialEq for Orderable<O> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<O: Ord> From<(u64, O)> for Orderable<O> {
    fn from(from: (u64, O)) -> Self {
        Orderable {
            id: from.0,
            ordering: from.1,
        }
    }
}
