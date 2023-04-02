use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

use lighthouse_types::{EthSpec, SignedBeaconBlock, Slot};

#[derive(Debug, Clone)]
pub enum BlockState<E: EthSpec> {
    Proposed(Arc<SignedBeaconBlock<E>>),
    Missed(Slot),
    Orphaned(Arc<SignedBeaconBlock<E>>),
}

impl<E: EthSpec> BlockState<E> {
    pub fn slot(&self) -> Slot {
        match self {
            BlockState::Proposed(block) => block.slot(),
            BlockState::Missed(s) => s.clone(),
            BlockState::Orphaned(block) => block.slot(),
        }
    }
}

impl<E: EthSpec> Display for BlockState<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockState::Proposed(_) => write!(f, "Proposed"),
            BlockState::Missed(_) => write!(f, "Missed"),
            BlockState::Orphaned(_) => write!(f, "Orphaned"),
        }
    }
}
