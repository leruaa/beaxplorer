use lighthouse_network::PeerId;
use lighthouse_types::{EthSpec, Hash256};
use types::utils::RequestAttempts;

use crate::db::EpochToPersist;

#[derive(Debug, Clone)]
pub enum Work<E: EthSpec> {
    PersistEpoch(EpochToPersist<E>),
    PersistBlockRequest(Hash256, RequestAttempts),
    PersistGoodPeers,
    SendRangeRequest(PeerId),
    SendBlockByRootRequest(PeerId, Hash256),
}
