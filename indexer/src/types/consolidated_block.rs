use types::{BeaconBlock, EthSpec};


#[derive(Debug)]
pub struct ConsolidatedBlock<E: EthSpec> {
    pub block: Option<BeaconBlock<E>>,
    pub status: Status,
    pub proposer: u64,
}

#[derive(Debug)]
pub enum Status {
    Scheduled = 0,
    Proposed = 1,
    Missed = 2,
    Orphaned = 3,
}

impl<E: EthSpec> ConsolidatedBlock<E> {

    pub fn new(block: Option<BeaconBlock<E>>, status: Status, proposer: u64) -> Self {
        ConsolidatedBlock {
            block,
            status,
            proposer,
        }
    }
}