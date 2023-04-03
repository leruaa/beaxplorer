use std::collections::HashMap;

use lighthouse_network::PeerId;
use lighthouse_types::{Epoch, EthSpec, Hash256, Slot};
use types::utils::RequestAttempts;

use crate::types::block_state::BlockState;

#[derive(Debug, Clone)]
pub enum Work<E: EthSpec> {
    PersistEpoch {
        epoch: Epoch,
        blocks: HashMap<Slot, BlockState<E>>,
    },
    PersistBlock(BlockState<E>),
    PersistBlockRequest(Hash256, RequestAttempts),
    PersistAllBlockRequests,
    PersistAllGoodPeers,
    SendRangeRequest(Option<PeerId>),
    SendBlockByRootRequest(Hash256, PeerId),
}
