use lighthouse_types::{EthSpec, Hash256};
use types::utils::RequestAttempts;

use crate::types::{consolidated_block::ConsolidatedBlock, consolidated_epoch::ConsolidatedEpoch};

#[derive(Debug)]
pub enum Work<E: EthSpec> {
    PersistIndexingState(),
    PersistBlock(ConsolidatedBlock<E>),
    PersistEpoch(ConsolidatedEpoch<E>),
    PersistBlockRequest(Hash256, RequestAttempts),
    PersistAllBlockRequests,
    PersistAllGoodPeers,
}
