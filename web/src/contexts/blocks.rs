use std::convert::TryInto;

use db::models::BlockModel;
use serde::Serialize;
use types::EthSpec;

use crate::views::block::BlockView;

#[derive(Serialize)]
pub struct BlocksContext<E: EthSpec> {
    pub blocks: Vec<Option<BlockView<E>>>,
}

impl<E: EthSpec> BlocksContext<E> {
    pub fn new(blocks: Vec<BlockModel>) -> Self {
        BlocksContext {
            blocks: blocks.into_iter().map(|e| e.try_into().ok()).collect(),
        }
    }
}
