use types::{BeaconBlock, Epoch, EthSpec, Validator};

#[derive(Debug)]
pub struct ConsolidatedEpoch<E: EthSpec> {
    pub epoch: Epoch,
    pub blocks: Vec<BeaconBlock<E>>,
    pub validators: Vec<Validator>,
}

impl<E: EthSpec> ConsolidatedEpoch<E> {
    pub fn new(epoch: Epoch) -> Self {
        ConsolidatedEpoch {
            epoch,
            blocks: vec!(),
            validators: vec!(),
        }
    }
}