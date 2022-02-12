use serde::Serialize;
use types::block::{BlockExtendedModel, BlockModel, BlockModelWithId};

#[derive(Serialize, Debug, Clone)]
pub struct BlockView {
    pub slot: u64,
    #[serde(flatten)]
    pub model: BlockModel,
}

impl From<BlockModelWithId> for BlockView {
    fn from(value: BlockModelWithId) -> Self {
        BlockView {
            slot: value.id,
            model: value.model,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct BlockExtendedView {
    pub slot: u64,
    #[serde(flatten)]
    pub model: BlockModel,
    #[serde(flatten)]
    pub extended_model: BlockExtendedModel,
}

impl From<(u64, BlockModel, BlockExtendedModel)> for BlockExtendedView {
    fn from((slot, model, extended_model): (u64, BlockModel, BlockExtendedModel)) -> Self {
        BlockExtendedView {
            slot,
            model,
            extended_model,
        }
    }
}
