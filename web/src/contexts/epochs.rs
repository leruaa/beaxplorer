use std::convert::TryInto;

use db::models::EpochModel;
use serde::Serialize;
use types::EthSpec;

use crate::{contexts::common::breadcrumb::BreadcrumbPart, views::epoch::EpochView};

use super::common::breadcrumb::Breadcrumb;

#[derive(Serialize)]
pub struct EpochsContext<E: EthSpec> {
    pub breadcrumb: Breadcrumb,
    pub epochs: Vec<EpochView<E>>,
    pub pages_count: i64,
}

impl<E: EthSpec> EpochsContext<E> {
    pub fn new(epochs: Vec<EpochModel>, pages_count: i64) -> Self {
        EpochsContext {
            breadcrumb: vec![BreadcrumbPart::from_text_with_icon("Epochs", "clock")].into(),
            epochs: epochs
                .into_iter()
                .map(|e| e.try_into().ok())
                .filter_map(|e| e)
                .collect(),
            pages_count,
        }
    }
}
