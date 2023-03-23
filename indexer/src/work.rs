use lighthouse_network::PeerId;
use lighthouse_types::{EthSpec, Hash256};

use crate::db::blocks_by_epoch::EpochToPersist;

#[derive(Debug, Clone)]
pub enum Work<E: EthSpec> {
    PersistEpoch(EpochToPersist<E>),
    SendRangeRequest(PeerId),
    SendBlockByRootRequest(PeerId, Hash256),
    SendNetworkMessage(Option<PeerId>),
}
