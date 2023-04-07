use lighthouse_network::PeerId;
use lighthouse_types::{EthSpec, Hash256};
use types::utils::RequestAttempts;

use crate::types::block_state::BlockState;

#[derive(Debug, Clone)]
pub enum Work<E: EthSpec> {
    PersistBlock(BlockState<E>),
    PersistBlockRequest(Hash256, RequestAttempts),
    PersistAllBlockRequests,
    PersistAllGoodPeers,
    SendRangeRequest(Option<PeerId>),
    SendBlockByRootRequest(Hash256, PeerId),
}
