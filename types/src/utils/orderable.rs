use std::cmp::Ordering;

#[derive(Eq)]
pub struct Orderable<Id: Ord, O: Ord> {
    pub id: Id,
    pub ordering: O,
}

impl<Id: Ord, O: Ord> Ord for Orderable<Id, O> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.ordering.cmp(&other.ordering) {
            Ordering::Equal => self.id.cmp(&other.id),
            o => o,
        }
    }
}

impl<Id: Ord, O: Ord> PartialOrd for Orderable<Id, O> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Id: Ord, O: Ord> PartialEq for Orderable<Id, O> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<Id: Ord, O: Ord> From<(Id, O)> for Orderable<Id, O> {
    fn from(from: (Id, O)) -> Self {
        Orderable {
            id: from.0,
            ordering: from.1,
        }
    }
}
