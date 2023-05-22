use std::ops::Range;

use eth1::{DepositCache, SszDepositCache, DepositLog};
use lighthouse_types::{BeaconState, ChainSpec, EthSpec, RelativeEpoch, Slot, Deposit};
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeSeq, de::Visitor};
use ssz::{Encode, Decode};
use state_processing::{
    per_block_processing::{per_block_processing, process_operations::process_deposit, get_existing_validator_index}, per_epoch_processing, per_slot_processing, BlockSignatureStrategy,
    ConsensusContext, VerifyBlockRoot,
};
use types::{persistable::{MsgPackSerializable, ResolvablePersistable}, path::ToPath};

use crate::{
    types::{
        block_state::BlockState,
        consolidated_block::ConsolidatedBlock,
        consolidated_epoch::{AggregatedEpochData, ConsolidatedEpoch},
    },
};

#[derive(Serialize, Deserialize)]
pub struct IndexingState<E: EthSpec> {
    is_genesis: bool,
    aggregated_epoch_data: AggregatedEpochData,
    pub(super) beacon_state: BeaconState<E>,
    #[serde(serialize_with = "serialize_deposit_cache")]
    #[serde(deserialize_with = "deserialize_deposit_cache")]
    deposit_cache: DepositCache,
    #[serde(skip_serializing)]
    #[serde(deserialize_with = "spec::<_, E>")]
    pub(super) spec: ChainSpec,
}

impl<E: EthSpec> IndexingState<E> {
    pub fn new(genesis_state: BeaconState<E>, deposit_contract_deploy_block: u64) -> Self {
        Self {
            is_genesis: true,
            aggregated_epoch_data: AggregatedEpochData::default(),
            beacon_state: genesis_state,
            deposit_cache: DepositCache::new(deposit_contract_deploy_block),
            spec: E::default_spec(),
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
                beacon_state.previous_epoch(),
                self.aggregated_epoch_data.aggregate(),
                &s,
                beacon_state.balances().to_owned().into(),
            )
        });

        self.aggregated_epoch_data.consolidate(&block);

        let committees = if slot == 0 {
            beacon_state
                .committee_cache(RelativeEpoch::Previous)
                .map_err(|_| "The genesis committee cache has not been initialized".to_string())?
                .get_beacon_committees_at_slot(slot)
                .map_err(|_| "The committees at slot 0 are not in the cache".to_string())?
                .into_iter()
                .map(|c| c.into_owned())
                .collect()
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
            vec![],
        );

        self.beacon_state = beacon_state;
        self.is_genesis = false;

        Ok((consolidated_block, consolidated_epoch))
    }

    pub fn insert_deposits(&mut self, deposits_logs: Vec<DepositLog>) -> Result<Vec<DepositLog>, (String, Vec<DepositLog>)> {
        deposits_logs
            .into_iter()
            .try_fold(vec![], |mut acc, d| {
                match self
                    .deposit_cache
                    .insert_log(d.clone()) {
                        Ok(_) => {
                            acc.push(d);
                            Ok(acc)
                        },
                        Err(err) => Err((format!("Failed to insert log: {:?}", err), acc)),
                    }
            })
    }

    pub fn get_deposits(&mut self, range: Range<u64>) -> Result<Vec<Deposit>, String> {
        let (_, deposits) = self
            .deposit_cache
            .get_deposits(range.start, range.end, range.end)
            .map_err(|err| format!("Failed to retrieve deposits: {:?}", err))?;

        Ok(deposits)
    }

    pub fn process_deposit(&mut self, deposit: &Deposit, index: u64) -> Result<u64, String> {
        if index > self.beacon_state.eth1_deposit_index() {
            process_deposit(&mut self.beacon_state, deposit, &self.spec, true)
                .map_err(|err| format!("Failed to process deposit '{}': {:?}", index, err))?;
        }
        
        let validator_index = get_existing_validator_index(&mut self.beacon_state, &deposit.data.pubkey)
            .map_err(|err| format!("Failed to find validator for deposit '{}': {:?}", index, err))?;

        validator_index.ok_or(format!("Failed to find validator for deposit '{}'", index))
    }
}


impl<E: EthSpec + Serialize> MsgPackSerializable for IndexingState<E> {}

impl <E: EthSpec + Serialize> ResolvablePersistable for IndexingState<E> {
    fn save(&self, base_path: &str) -> Result<(), String> {
        let full_path = Self::to_path(base_path, &());
        self.serialize_to_file(&full_path)
    }
}

impl<E: EthSpec> ToPath for IndexingState<E> {
    type Id = ();

    fn to_path(base_dir: &str, _: &Self::Id) -> String {
        format!("{}/indexing_state.msg", base_dir)
    }
}

fn spec<'de, D, E: EthSpec>(_: D) -> Result<ChainSpec, D::Error> where D: Deserializer<'de> {
    Ok(E::default_spec())
}

fn serialize_deposit_cache<S>(deposit_cache: &DepositCache, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    let ssz_deposit_cache = SszDepositCache::from_deposit_cache(deposit_cache);

    let bytes = ssz_deposit_cache.as_ssz_bytes();

    let mut seq = serializer.serialize_seq(Some(bytes.len()))?;

    for b in bytes {
        seq.serialize_element(&b)?
    }

    seq.end()
}

fn deserialize_deposit_cache<'de, D>(deserializer: D) -> Result<DepositCache, D::Error> where D: Deserializer<'de> {
    struct DepositCacheVisitor;

    impl<'de> Visitor<'de> for DepositCacheVisitor {
        type Value = DepositCache;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a ssz deposit cache")
        }

        fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where A: serde::de::SeqAccess<'de>,
        {
            let bytes = Vec::with_capacity(seq.size_hint().unwrap_or(0));

            let ssz_deposit_cache = SszDepositCache::from_ssz_bytes(&bytes).map_err(|err| serde::de::Error::custom(format!("Failed to decode deposit cache: {:?}", err)))?;

            ssz_deposit_cache.to_deposit_cache().map_err(serde::de::Error::custom)
        }
    }

    deserializer.deserialize_seq(DepositCacheVisitor)
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
    async fn test_genesis_deposits() {
        let harness = BeaconChainHarness::new();

        assert_eq!(harness.state().validators().len(), 2);
    }

    #[tokio::test]
    async fn test_contains_block_root() {
        let mut harness = BeaconChainHarness::new();
        let beacon_context =
            BeaconContext::<MainnetEthSpec>::new(harness.state(), harness.spec()).unwrap();
        let mut indexing_state = IndexingState::new(beacon_context.genesis_state, beacon_context.eth2_network_config.deposit_contract_deploy_block);

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
