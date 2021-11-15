use serde::Serialize;

use crate::contexts::common::breadcrumb::BreadcrumbPart;

use super::common::breadcrumb::Breadcrumb;

#[derive(Serialize)]
pub struct BlocksContext {
    pub breadcrumb: Breadcrumb,
}

impl BlocksContext {
    pub fn new() -> Self {
        BlocksContext {
            breadcrumb: vec![BreadcrumbPart::from_text_with_icon("Blocks", "cube")].into(),
        }
    }
}
