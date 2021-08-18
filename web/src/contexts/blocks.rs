use std::convert::TryInto;

use db::models::BlockModel;
use serde::Serialize;
use types::EthSpec;

use crate::{contexts::common::breadcrumb::BreadcrumbPart, views::block::BlockView};

use super::common::breadcrumb::Breadcrumb;

#[derive(Serialize)]
pub struct BlocksContext<E: EthSpec> {
    pub breadcrumb: Breadcrumb,
    pub blocks: Vec<Option<BlockView<E>>>,
}

impl<E: EthSpec> BlocksContext<E> {
    pub fn new(blocks: Vec<BlockModel>) -> Self {
        BlocksContext {
            breadcrumb: vec![BreadcrumbPart::from_text("Blocks")].into(),
            blocks: blocks.into_iter().map(|e| e.try_into().ok()).collect(),
        }
    }
}
