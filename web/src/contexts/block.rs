use std::convert::TryInto;

use db::models::BlockModel;
use serde::Serialize;
use types::EthSpec;

use crate::views::block::BlockView;

#[derive(Serialize)]
pub struct BlockContext<E: EthSpec> {
    pub block: Option<BlockView<E>>,
}

impl<E: EthSpec> BlockContext<E> {
    pub fn new(block: BlockModel) -> Self {
        BlockContext {
            block: block.try_into().ok(),
        }
    }
}
