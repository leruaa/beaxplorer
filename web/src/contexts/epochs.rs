use std::convert::TryInto;

use db::models::EpochModel;
use serde::Serialize;
use types::EthSpec;

use crate::{contexts::common::breadcrumb::BreadcrumbPart, views::epoch::EpochView};

use super::common::breadcrumb::Breadcrumb;

#[derive(Serialize)]
pub struct EpochsContext<E: EthSpec> {
    pub breadcrumb: Breadcrumb,
    pub epochs: Vec<Option<EpochView<E>>>,
}

impl<E: EthSpec> EpochsContext<E> {
    pub fn new(epochs: Vec<EpochModel>) -> Self {
        EpochsContext {
            breadcrumb: vec![BreadcrumbPart::from_text("Epochs")].into(),
            epochs: epochs.into_iter().map(|e| e.try_into().ok()).collect(),
        }
    }
}
