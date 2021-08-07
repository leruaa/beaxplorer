use db::models::BlockModel;
use serde::Serialize;

use crate::views::block::BlockView;

#[derive(Serialize)]
pub struct BlockContext {
    pub block: BlockView,
}

impl BlockContext {
    pub fn new(block: BlockModel) -> Self {
        BlockContext {
            block: block.into(),
        }
    }
}
