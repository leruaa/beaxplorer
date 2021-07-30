use db::models::EpochModel;
use serde::Serialize;

use crate::views::epoch::EpochView;

#[derive(Serialize)]
pub struct EpochsContext {
    pub epochs: Vec<EpochView>,
}

impl EpochsContext {
    pub fn new(epochs: Vec<EpochModel>) -> Self {
        EpochsContext {
            epochs: epochs.into_iter().map(|e| e.into()).collect(),
        }
    }
}
