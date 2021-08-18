use std::convert::TryInto;

use db::models::EpochModel;
use serde::Serialize;
use types::EthSpec;

use crate::{contexts::common::breadcrumb::BreadcrumbPart, views::epoch::EpochView};

use super::common::breadcrumb::Breadcrumb;

#[derive(Serialize)]
pub struct EpochContext<E: EthSpec> {
    pub breadcrumb: Breadcrumb,
    pub epoch: Option<EpochView<E>>,
}

impl<E: EthSpec> EpochContext<E> {
    pub fn new(epoch: EpochModel) -> Self {
        EpochContext {
            breadcrumb: vec![
                BreadcrumbPart::from_link("Epochs", "/epochs"),
                epoch.clone().into(),
            ]
            .into(),
            epoch: epoch.try_into().ok(),
        }
    }
}
