use db::models::BlockModel;
use serde::Serialize;

#[derive(Serialize, Default)]
pub struct BlockView {
    pub epoch: String,
    pub slot: String,
    pub proposer: String,
    pub attestations_count: String,
}

impl From<BlockModel> for BlockView {
    fn from(model: BlockModel) -> Self {
        BlockView {
            epoch: model.epoch.to_string(),
            slot: model.slot.to_string(),
            proposer: model.proposer.to_string(),
            attestations_count: model.attestations_count.to_string(),
        }
    }
}
