use serde::Serialize;

use crate::contexts::common::breadcrumb::BreadcrumbPart;

use super::common::breadcrumb::Breadcrumb;

#[derive(Serialize)]
pub struct EpochsContext {
    pub breadcrumb: Breadcrumb,
    pub pages_count: i64,
}

impl EpochsContext {
    pub fn new(pages_count: i64) -> Self {
        EpochsContext {
            breadcrumb: vec![BreadcrumbPart::from_text_with_icon("Epochs", "clock")].into(),
            pages_count,
        }
    }
}
