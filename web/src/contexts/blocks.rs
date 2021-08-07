use db::models::BlockModel;
use serde::Serialize;

use crate::views::block::BlockView;

#[derive(Serialize)]
pub struct BlocksContext {
    pub blocks: Vec<BlockView>,
}

impl BlocksContext {
    pub fn new(blocks: Vec<BlockModel>) -> Self {
        BlocksContext {
            blocks: blocks.into_iter().map(|e| e.into()).collect(),
        }
    }
}
