use std::sync::Arc;

use lighthouse_types::EthSpec;
use parking_lot::RwLock;
use task_executor::TaskExecutor;

use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedSender},
    watch::Receiver,
};
use tracing::{debug, error, info, instrument};
use types::{
    attestation::AttestationModelsWithId,
    block::{BlockExtendedModelWithId, BlockModelWithId, BlocksMeta},
    block_root::BlockRootModelWithId,
    committee::CommitteeModelsWithId,
    persistable::Persistable,
    utils::PersistableCache,
    vote::{VoteModel, VoteModelsWithId},
};

use crate::{
    db::{BlockRootsCache, Stores},
    types::consolidated_block::ConsolidatedBlock,
};

pub fn spawn_persist_block_worker<E: EthSpec>(
    base_dir: String,
    stores: Arc<Stores<E>>,
    mut shutdown_trigger: Receiver<()>,
    executor: &TaskExecutor,
) -> UnboundedSender<ConsolidatedBlock<E>> {
    let (new_block_send, mut new_block_recv) = unbounded_channel();

    let mut votes_cache = PersistableCache::new(base_dir.clone());

    executor.spawn(
        async move {
            loop {
                tokio::select! {
                    Some(block) = new_block_recv.recv() => {
                        persist_block::<E>(&base_dir, block, stores.block_roots_cache(), &mut votes_cache);
                    }

                    _ = shutdown_trigger.changed() => {
                        info!("Shutting down blocks worker...");
                        return;
                    }
                }
            }
        },
        "persist block worker",
    );

    new_block_send
}

#[instrument(name = "BlockPersist", skip_all)]
fn persist_block<E: EthSpec>(
    base_dir: &str,
    block: ConsolidatedBlock<E>,
    block_roots_cache: Arc<RwLock<BlockRootsCache>>,
    votes_cache: &mut PersistableCache<VoteModelsWithId>,
) {
    debug!(slot = %block.slot(), "Persisting block");
    let mut block_roots_cache = block_roots_cache.write();

    BlockModelWithId::from(&block).persist(base_dir);
    BlockExtendedModelWithId::from(&block).persist(base_dir);
    AttestationModelsWithId::from(&block).persist(base_dir);
    CommitteeModelsWithId::from(&block).persist(base_dir);
    BlockRootModelWithId::from(&block).persist(base_dir);

    block.attestations().iter().for_each(|attestation| {
        match block_roots_cache.get(attestation.data.beacon_block_root) {
            Some(s) => {
                votes_cache
                    .get_or_default_mut(s.as_u64())
                    .model
                    .push(VoteModel::from(&attestation.data));
            }
            None => error!(
                "Attestation found with unknown root '{}'",
                attestation.data.beacon_block_root
            ),
        }
    });

    votes_cache.persist_dirty();

    BlocksMeta::new(block.slot().as_usize() + 1).persist(base_dir);
}
