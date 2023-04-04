use std::fmt::Display;

use lighthouse_network::PeerId;
use lighthouse_types::{EthSpec, Hash256, Slot};

use crate::types::block_state::BlockState;

#[derive(Debug, Clone)]
pub enum NetworkEvent<E: EthSpec> {
    PeerConnected(PeerId),
    PeerDisconnected(PeerId),
    RangeRequestSuccedeed,
    RangeRequestFailed(PeerId),
    BlockRequestFailed(Hash256, PeerId),
    NewBlock(BlockState<E>, PeerId),
    UnknownBlockRoot(Slot, Hash256),
    BlockRootFound(Hash256, Slot, PeerId),
    BlockRootNotFound(Hash256),
}

impl<E: EthSpec> Display for NetworkEvent<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkEvent::PeerConnected(_) => write!(f, "Peer connected"),
            NetworkEvent::PeerDisconnected(_) => write!(f, "Peer disconnected"),
            NetworkEvent::RangeRequestSuccedeed => write!(f, "Range request succedeed"),
            NetworkEvent::RangeRequestFailed(_) => write!(f, "Range request failed"),
            NetworkEvent::BlockRequestFailed(_, _) => write!(f, "Block request failed"),
            NetworkEvent::NewBlock(block, _) => match block {
                BlockState::Proposed(block) => {
                    write!(f, "New proposed block at {}", block.slot())
                }
                BlockState::Missed(slot) => write!(f, "New missed block at {}", slot),
                BlockState::Orphaned(block) => {
                    write!(f, "New orphaned block at {}", block.slot())
                }
            },
            NetworkEvent::UnknownBlockRoot(_, _) => write!(f, "Unknown block root"),
            NetworkEvent::BlockRootFound(_, _, _) => write!(f, "Block root found"),
            NetworkEvent::BlockRootNotFound(_) => write!(f, "Block root not found"),
        }
    }
}
