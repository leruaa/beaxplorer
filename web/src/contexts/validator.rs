use std::convert::TryInto;

use db::models::ValidatorModel;
use serde::Serialize;
use types::EthSpec;

use crate::{contexts::common::breadcrumb::BreadcrumbPart, views::validator::ValidatorView};

use super::common::breadcrumb::Breadcrumb;

#[derive(Serialize)]
pub struct ValidatorContext<E: EthSpec> {
    pub breadcrumb: Breadcrumb,
    pub validator: Option<ValidatorView<E>>,
}

impl<E: EthSpec> ValidatorContext<E> {
    pub fn new(validator: ValidatorModel) -> Self {
        ValidatorContext {
            breadcrumb: vec![
                BreadcrumbPart::from_link("Validators", "/validators", "users"),
                validator.clone().into(),
            ]
            .into(),
            validator: validator.try_into().ok(),
        }
    }
}
