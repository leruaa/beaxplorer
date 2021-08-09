use std::convert::TryInto;

use db::models::EpochModel;
use serde::Serialize;
use types::EthSpec;

use crate::views::epoch::EpochView;

#[derive(Serialize)]
pub struct EpochsContext<E: EthSpec> {
    pub epochs: Vec<Option<EpochView<E>>>,
}

impl<E: EthSpec> EpochsContext<E> {
    pub fn new(epochs: Vec<EpochModel>) -> Self {
        EpochsContext {
            epochs: epochs.into_iter().map(|e| e.try_into().ok()).collect(),
        }
    }
}
