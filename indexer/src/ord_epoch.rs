use std::cmp::Ordering;

use db::models::EpochModel;

#[derive(Eq)]
pub struct EpochWithAttestationsCount {
    pub epoch: i64,
    pub attestations_count: i32,
}

impl Ord for EpochWithAttestationsCount {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.attestations_count.cmp(&other.attestations_count)
    }
}

impl PartialOrd for EpochWithAttestationsCount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for EpochWithAttestationsCount {
    fn eq(&self, other: &Self) -> bool {
        self.epoch == other.epoch
    }
}

impl From<&EpochModel> for EpochWithAttestationsCount {
    fn from(model: &EpochModel) -> Self {
        EpochWithAttestationsCount {
            epoch: model.epoch,
            attestations_count: model.attestations_count,
        }
    }
}
