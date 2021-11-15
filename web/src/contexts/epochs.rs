use serde::Serialize;

use crate::contexts::common::breadcrumb::BreadcrumbPart;

use super::common::breadcrumb::Breadcrumb;

#[derive(Serialize)]
pub struct EpochsContext {
    pub breadcrumb: Breadcrumb,
}

impl EpochsContext {
    pub fn new() -> Self {
        EpochsContext {
            breadcrumb: vec![BreadcrumbPart::from_text_with_icon("Epochs", "clock")].into(),
        }
    }
}
