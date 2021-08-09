use std::convert::TryInto;

use db::models::EpochModel;
use serde::Serialize;
use types::EthSpec;

use crate::views::epoch::EpochView;

#[derive(Serialize)]
pub struct EpochContext<E: EthSpec> {
    pub epoch: Option<EpochView<E>>,
}

impl<E: EthSpec> EpochContext<E> {
    pub fn new(epoch: EpochModel) -> Self {
        EpochContext {
            epoch: epoch.try_into().ok(),
        }
    }
}
