use lighthouse_network::PeerId;
use lighthouse_types::{EthSpec, Hash256, Slot};

use crate::types::block_state::BlockState;

#[derive(Debug, Clone)]
pub enum NetworkEvent<E: EthSpec> {
    PeerConnected(PeerId),
    PeerDisconnected(PeerId),
    RangeRequestSuccedeed,
    RangeRequestFailed,
    BlockRequestFailed(Hash256, PeerId),
    NewBlock(BlockState<E>),
    UnknownBlockRoot(Slot, Hash256),
    BlockRootFound(Hash256, Slot, PeerId),
    BlockRootNotFound(Hash256),
}
