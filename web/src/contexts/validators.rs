use serde::Serialize;

use crate::contexts::common::breadcrumb::BreadcrumbPart;

use super::common::breadcrumb::Breadcrumb;

#[derive(Serialize)]
pub struct ValidatorsContext {
    pub breadcrumb: Breadcrumb,
}

impl ValidatorsContext {
    pub fn new() -> Self {
        ValidatorsContext {
            breadcrumb: vec![BreadcrumbPart::from_text_with_icon("Validators", "users")].into(),
        }
    }
}
