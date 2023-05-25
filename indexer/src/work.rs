use lighthouse_types::{EthSpec, Hash256};
use types::utils::RequestAttempts;

use crate::types::{
    consolidated_block::ConsolidatedBlock, consolidated_epoch::ConsolidatedEpoch,
    consolidated_execution_layer_deposit::ConsolidatedExecutionLayerDeposit,
};

#[derive(Debug)]
pub enum Work<E: EthSpec> {
    PersistIndexingState(),
    PersistDepositFromExecutionLayer(ConsolidatedExecutionLayerDeposit),
    PersistBlock(ConsolidatedBlock<E>),
    PersistEpoch(ConsolidatedEpoch<E>),
    PersistBlockRequest(Hash256, RequestAttempts),
    PersistAllBlockRequests,
    PersistAllGoodPeers,
}
