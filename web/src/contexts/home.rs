use db::models::EpochModel;
use serde::Serialize;

use crate::views::epoch::EpochView;

#[derive(Serialize)]
pub struct HomeContext {
    pub epochs: Vec<EpochView>,
}

impl HomeContext {
    pub fn new(epochs: Vec<EpochModel>) -> Self {
        HomeContext {
            epochs: epochs.into_iter().map(|e| e.into()).collect(),
        }
    }
}
