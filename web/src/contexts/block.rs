use std::convert::TryInto;

use db::models::BlockModel;
use serde::Serialize;
use types::EthSpec;

use crate::{contexts::common::breadcrumb::BreadcrumbPart, views::block::BlockView};

use super::common::breadcrumb::Breadcrumb;

#[derive(Serialize)]
pub struct BlockContext<E: EthSpec> {
    pub breadcrumb: Breadcrumb,
    pub block: Option<BlockView<E>>,
}

impl<E: EthSpec> BlockContext<E> {
    pub fn new(block: BlockModel) -> Self {
        BlockContext {
            breadcrumb: vec![
                BreadcrumbPart::from_link("Blocks", "/blocks", "cube"),
                block.clone().into(),
            ]
            .into(),
            block: block.try_into().ok(),
        }
    }
}
