use std::convert::TryInto;

use db::models::EpochModel;
use serde::Serialize;
use types::EthSpec;

use crate::views::epoch::EpochView;

#[derive(Serialize)]
pub struct HomeContext<E: EthSpec> {
    pub latest_epochs: Vec<Option<EpochView<E>>>,
}

impl<E: EthSpec> HomeContext<E> {
    pub fn new(epochs: Vec<EpochModel>) -> Self {
        HomeContext {
            latest_epochs: epochs.into_iter().map(|e| e.try_into().ok()).collect(),
        }
    }
}
