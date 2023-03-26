use std::{collections::HashMap, sync::Arc};

use lighthouse_network::PeerId;
use lighthouse_types::{Epoch, EthSpec, Hash256, SignedBeaconBlock, Slot};
use types::utils::RequestAttempts;

use crate::types::block_state::BlockState;

#[derive(Debug, Clone)]
pub enum Work<E: EthSpec> {
    PersistEpoch {
        epoch: Epoch,
        blocks: HashMap<Slot, BlockState<E>>,
    },
    PersistBlock(Arc<SignedBeaconBlock<E>>),
    PersistBlockRequest(Hash256, RequestAttempts),
    PersistGoodPeers,
    SendRangeRequest(PeerId),
    SendBlockByRootRequest(PeerId, Hash256),
}
