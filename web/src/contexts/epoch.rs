use db::models::EpochModel;
use serde::Serialize;

use crate::views::epoch::EpochView;

#[derive(Serialize)]
pub struct EpochContext {
    pub epoch: EpochView,
}

impl EpochContext {
    pub fn new(epoch: EpochModel) -> Self {
        EpochContext {
            epoch: epoch.into(),
        }
    }
}
