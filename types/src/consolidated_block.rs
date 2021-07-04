use lighthouse_types::{BeaconBlock, EthSpec};


#[derive(Debug)]
pub struct ConsolidatedBlock<E: EthSpec> {
    pub block: Option<BeaconBlock<E>>,
    pub status: BlockStatus,
    pub proposer: u64,
}

#[derive(Debug)]
pub enum BlockStatus {
    Scheduled = 0,
    Proposed = 1,
    Missed = 2,
    Orphaned = 3,
}

impl<E: EthSpec> ConsolidatedBlock<E> {

    pub fn new(block: Option<BeaconBlock<E>>, status: BlockStatus, proposer: u64) -> Self {
        ConsolidatedBlock {
            block,
            status,
            proposer,
        }
    }
}