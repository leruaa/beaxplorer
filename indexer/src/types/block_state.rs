use std::sync::Arc;

use lighthouse_types::{EthSpec, SignedBeaconBlock, Slot};

#[derive(Debug, Clone)]
pub enum BlockState<E: EthSpec> {
    Proposed(Arc<SignedBeaconBlock<E>>),
    Missed(Slot),
    Orphaned(Arc<SignedBeaconBlock<E>>),
}
