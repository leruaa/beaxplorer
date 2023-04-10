use std::sync::Arc;

use lighthouse_types::{BeaconState, ChainSpec, EthSpec, Slot};
use state_processing::{
    per_block_processing, per_epoch_processing, per_slot_processing, BlockSignatureStrategy,
    ConsensusContext, VerifyBlockRoot,
};

use crate::{
    beacon_chain::beacon_context::BeaconContext,
    types::{
        block_state::BlockState,
        consolidated_block::ConsolidatedBlock,
        consolidated_epoch::{AggregatedEpochData, ConsolidatedEpoch},
    },
};

#[derive(Debug)]
pub struct IndexingState<E: EthSpec> {
    is_genesis: bool,
    aggregated_epoch_data: AggregatedEpochData,
    pub(super) beacon_state: BeaconState<E>,
    pub(super) spec: ChainSpec,
}

impl<E: EthSpec> IndexingState<E> {
    pub fn new(beacon_context: Arc<BeaconContext<E>>) -> Self {
        Self {
            is_genesis: true,
            aggregated_epoch_data: AggregatedEpochData::default(),
            beacon_state: beacon_context.genesis_state.clone(),
            spec: beacon_context.spec.clone(),
        }
    }

    pub fn latest_slot(&self) -> Option<Slot> {
        if self.is_genesis {
            None
        } else {
            Some(self.beacon_state.slot())
        }
    }

    pub fn can_process_slot(&self, slot: Slot) -> bool {
        match self.latest_slot() {
            Some(latest_slot) => slot > latest_slot,
            None => true,
        }
    }

    pub fn process_block(
        &mut self,
        block: BlockState<E>,
    ) -> Result<(ConsolidatedBlock<E>, Option<ConsolidatedEpoch<E>>), String> {
        let slot = block.slot();
        let mut beacon_state = self.beacon_state.clone();
        let mut consensus_context = ConsensusContext::new(block.slot());

        let summary = match &block {
            BlockState::Proposed(beacon_block) => {
                if block.slot() > 0 {
                    let summary = per_slot_processing(&mut beacon_state, None, &self.spec)
                        .map_err(|err| format!("Error while processing slot: {err:?}"))?;

                    per_block_processing(
                        &mut beacon_state,
                        beacon_block,
                        BlockSignatureStrategy::NoVerification,
                        VerifyBlockRoot::False,
                        &mut consensus_context,
                        &self.spec,
                    )
                    .map_err(|err| format!("Error while processing block: {err:?}"))?;

                    summary
                } else {
                    Some(per_epoch_processing(&mut beacon_state, &self.spec).unwrap())
                }
            }
            BlockState::Missed(_) => per_slot_processing(&mut beacon_state, None, &self.spec)
                .map_err(|err| format!("Error while processing slot: {err:?}"))?,
            BlockState::Orphaned(_) => None,
        };

        let consolidated_epoch = summary.map(|s| {
            ConsolidatedEpoch::new(
                block.epoch(),
                self.aggregated_epoch_data.clone(),
                &s,
                beacon_state.balances().to_owned().into(),
            )
        });

        let committees = if slot == 0 {
            vec![]
        } else {
            beacon_state
                .get_beacon_committees_at_slot(slot)
                .map_err(|err| format!("Error while processing committees: {err:?}"))?
                .into_iter()
                .map(|c| c.into_owned())
                .collect()
        };

        let consolidated_block = ConsolidatedBlock::new(
            block,
            consensus_context
                .get_proposer_index(&beacon_state, &self.spec)
                .map_err(|err| format!("Error while processing proposer: {err:?}"))?,
            committees,
        );

        self.beacon_state = beacon_state;
        self.is_genesis = false;

        Ok((consolidated_block, consolidated_epoch))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use lighthouse_types::{MainnetEthSpec, Slot};

    use crate::{
        beacon_chain::beacon_context::BeaconContext, db::indexing_state::IndexingState,
        test_utils::BeaconChainHarness, types::block_state::BlockState,
    };

    #[tokio::test]
    async fn test_contains_block_root() {
        let mut harness = BeaconChainHarness::new();
        let beacon_context =
            BeaconContext::<MainnetEthSpec>::new(harness.state(), harness.spec()).unwrap();
        let mut indexing_state = IndexingState::new(Arc::new(beacon_context));

        let at_0 = Arc::new(harness.make_block(0).await);
        let at_1 = Arc::new(harness.make_block(1).await);
        let at_2 = Arc::new(harness.make_block(2).await);
        let at_3 = Arc::new(harness.make_block(3).await);

        indexing_state
            .process_block(BlockState::Proposed(at_0.clone()))
            .unwrap();
        indexing_state
            .process_block(BlockState::Proposed(at_1.clone()))
            .unwrap();
        indexing_state
            .process_block(BlockState::Proposed(at_2.clone()))
            .unwrap();
        indexing_state
            .process_block(BlockState::Proposed(at_3.clone()))
            .unwrap();

        indexing_state
            .beacon_state
            .build_all_caches(&harness.spec())
            .unwrap();

        println!(
            "Canonical root: {} | {:?}",
            at_0.canonical_root(),
            indexing_state.beacon_state.get_block_root(Slot::new(0))
        );
        println!(
            "Canonical root: {} | {:?}",
            at_1.canonical_root(),
            indexing_state.beacon_state.get_block_root(Slot::new(1))
        );
        println!(
            "Canonical root: {} | {:?}",
            at_2.canonical_root(),
            indexing_state.beacon_state.get_block_root(Slot::new(2))
        );
        println!(
            "Canonical root: {} | {:?}",
            at_3.canonical_root(),
            indexing_state.beacon_state.get_block_root(Slot::new(3))
        );
    }
}
