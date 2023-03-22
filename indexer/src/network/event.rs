use std::sync::Arc;

use lighthouse_network::PeerId;
use lighthouse_types::{EthSpec, Hash256, SignedBeaconBlock, Slot};

#[derive(Debug, Clone)]
pub enum NetworkEvent<E: EthSpec> {
    PeerConnected(PeerId),
    PeerDisconnected(PeerId),
    RangeRequestSuccedeed(u64),
    RangeRequestFailed(u64),
    BlockRequestFailed(Hash256, PeerId),
    ProposedBlock(Arc<SignedBeaconBlock<E>>, Slot),
    OrphanedBlock(Arc<SignedBeaconBlock<E>>),
    MissedBlock(Slot),
    UnknownBlockRoot(Slot, Hash256),
    BlockRootFound(Hash256, PeerId),
    BlockRootNotFound(Hash256),
}
