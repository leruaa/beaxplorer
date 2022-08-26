use std::{
    collections::{hash_map::Entry, HashMap},
    convert::TryFrom,
    pin::Pin,
    rc::Rc,
    sync::Arc,
    time::Duration,
};

use futures::Future;
use lighthouse_network::NetworkGlobals;

use lighthouse_types::{BeaconState, BlindedPayload, ChainSpec};
use slog::{info, Logger};
use state_processing::{
    per_block_processing, per_epoch_processing::process_epoch, per_slot_processing,
    BlockReplayError, BlockSignatureStrategy, VerifyBlockRoot,
};
use store::{Epoch, EthSpec, MainnetEthSpec, SignedBeaconBlock, Slot};
use tokio::{
    sync::mpsc::UnboundedReceiver,
    time::{interval_at, Instant},
};
use types::{
    block::{BlockExtendedModelWithId, BlockModelWithId, BlocksMeta},
    epoch::{EpochExtendedModelWithId, EpochModelWithId, EpochsMeta},
};

use crate::{
    persistable::Persistable,
    types::{
        consolidated_block::{BlockStatus, ConsolidatedBlock},
        consolidated_epoch::ConsolidatedEpoch,
    },
};

// use the executor for libp2p
struct Executor(task_executor::TaskExecutor);

impl libp2p::core::Executor for Executor {
    fn exec(&self, f: Pin<Box<dyn Future<Output = ()> + Send>>) {
        self.0.spawn(f, "libp2p");
    }
}

#[derive(Debug, Clone)]
pub enum BlockMessage<E: EthSpec> {
    Proposed(Box<SignedBeaconBlock<E>>),
    Orphaned(Box<SignedBeaconBlock<E>>),
    MaybeOrphaned(Slot),
    Missed(Slot),
}

pub struct Indexer<E: EthSpec> {
    base_dir: String,
    beacon_state: BeaconState<E>,
    spec: ChainSpec,
    blocks_by_epoch: HashMap<Epoch, HashMap<Slot, BlockMessage<E>>>,
    log: Logger,
}

impl<E: EthSpec> Indexer<E> {
    pub fn new(
        base_dir: String,
        beacon_state: BeaconState<E>,
        spec: ChainSpec,
        log: Logger,
    ) -> Self {
        Indexer {
            base_dir,
            beacon_state,
            spec,
            blocks_by_epoch: HashMap::new(),
            log,
        }
    }

    pub fn spawn_notifier(
        &self,
        executor: &task_executor::TaskExecutor,
        network_globals: Arc<NetworkGlobals<MainnetEthSpec>>,
    ) {
        let start_instant = Instant::now();
        let interval_duration = Duration::from_secs(5);
        let mut interval = interval_at(start_instant, interval_duration);
        let log = self.log.clone();

        executor.spawn(
            async move {
                loop {
                    interval.tick().await;

                    info!(log, "Status"; "connected peers" => network_globals.connected_peers());
                }
            },
            "notifier",
        );
    }

    pub fn spawn_indexer(
        mut self,
        executor: &task_executor::TaskExecutor,
        mut block_recv: UnboundedReceiver<BlockMessage<E>>,
    ) {
        executor.spawn(
            async move {
                loop {
                    if let Some(block_message) = block_recv.recv().await {
                        let slot = match &block_message {
                            BlockMessage::Proposed(block) => block.message().slot(),
                            BlockMessage::Orphaned(block) => block.message().slot(),
                            BlockMessage::MaybeOrphaned(slot) => *slot,
                            BlockMessage::Missed(slot) => *slot,
                        };

                        if slot.as_u64() == 0 || slot > self.beacon_state.slot() {
                            let epoch = slot.epoch(MainnetEthSpec::slots_per_epoch());

                            let blocks_by_slot = self
                                .blocks_by_epoch
                                .entry(epoch)
                                .or_insert_with(HashMap::new);

                            match blocks_by_slot.entry(slot) {
                                Entry::Occupied(mut e) => {
                                    if let BlockMessage::MaybeOrphaned(_) = e.get() {
                                        e.insert(block_message);
                                    }
                                }
                                Entry::Vacant(e) => {
                                    e.insert(block_message);
                                }
                            };

                            if blocks_by_slot.len() as u64 == MainnetEthSpec::slots_per_epoch() {
                                if let Some(blocks_by_slot) = self.blocks_by_epoch.remove(&epoch) {
                                    if let Ok(mut blocks_by_slot) = blocks_by_slot
                                        .iter()
                                        .map(|(s, b)| BlockStatus::try_from(b).map(|b| (s, b)))
                                        .collect::<Result<Vec<_>, _>>()
                                    {
                                        blocks_by_slot.sort_by(|(a, _), (b, _)| a.cmp(b));

                                        self.persist_epoch(&epoch, blocks_by_slot);
                                    } else {
                                        self.blocks_by_epoch.insert(epoch, blocks_by_slot);
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "indexer",
        );
    }

    fn persist_epoch(&mut self, epoch: &Epoch, blocks: Vec<(&Slot, BlockStatus<E>)>) {
        info!(self.log, "Persisting epoch {epoch}");

        let b = blocks
            .iter()
            .filter_map(|(_, b)| match b {
                BlockStatus::Proposed(b) => Some(b),
                _ => None,
            })
            .map(|b| {
                let b = *b.clone();
                let (b, _) = b.into();
                b
            })
            .collect::<Vec<_>>();

        let last_slot = epoch.end_slot(MainnetEthSpec::slots_per_epoch());

        self.apply_blocks(b, last_slot).unwrap();

        let summary = process_epoch(&mut self.beacon_state.clone(), &self.spec).unwrap();

        let blocks = blocks
            .into_iter()
            .map(|(slot, block_status)| {
                ConsolidatedBlock::new(
                    block_status,
                    *slot,
                    *epoch,
                    self.beacon_state
                        .get_beacon_proposer_index(*slot, &self.spec)
                        .unwrap() as u64,
                )
            })
            .collect::<Vec<_>>();

        let blocks = Rc::new(blocks);

        let block_models = blocks
            .iter()
            .map(BlockModelWithId::from)
            .collect::<Vec<_>>();

        let extended_block_models = blocks
            .iter()
            .map(BlockExtendedModelWithId::from)
            .collect::<Vec<_>>();

        let consolidated_epoch = ConsolidatedEpoch::new(
            *epoch,
            blocks.clone(),
            summary,
            self.beacon_state.balances().clone().into(),
        );

        let epoch_model = EpochModelWithId::from(&consolidated_epoch);

        let extended_epoch_model = EpochExtendedModelWithId::from(&consolidated_epoch);

        EpochsMeta::new(epoch.as_usize()).persist(&self.base_dir);
        BlocksMeta::new(blocks.len()).persist(&self.base_dir);

        epoch_model.persist(&self.base_dir);
        extended_epoch_model.persist(&self.base_dir);
        block_models.persist(&self.base_dir);
        extended_block_models.persist(&self.base_dir);
    }

    fn apply_blocks(
        &mut self,
        blocks: Vec<SignedBeaconBlock<E, BlindedPayload<E>>>,
        target_slot: Slot,
    ) -> Result<(), BlockReplayError> {
        for (i, block) in blocks.iter().enumerate() {
            // Allow one additional block at the start which is only used for its state root.
            if i == 0 && block.slot() <= self.beacon_state.slot() {
                continue;
            }

            while self.beacon_state.slot() < block.slot() {
                per_slot_processing(&mut self.beacon_state, None, &self.spec)
                    .map_err(BlockReplayError::from)?;
            }

            per_block_processing(
                &mut self.beacon_state,
                block,
                None,
                BlockSignatureStrategy::NoVerification,
                VerifyBlockRoot::False,
                &self.spec,
            )
            .map_err(BlockReplayError::from)?;
        }

        while self.beacon_state.slot() < target_slot {
            per_slot_processing(&mut self.beacon_state, None, &self.spec)
                .map_err(BlockReplayError::from)?;
        }

        Ok(())
    }
}
