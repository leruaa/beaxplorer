use std::sync::Arc;

use lighthouse_types::EthSpec;
use parking_lot::RwLock;
use state_processing::common::get_attesting_indices;
use task_executor::TaskExecutor;

use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedSender},
    watch::Receiver,
};
use tracing::{debug, error, info, instrument};
use types::{
    attestation::AttestationModelsWithId,
    block::{BlockExtendedModel, BlockExtendedModelWithId, BlockModelWithId, BlocksMeta},
    block_root::{BlockRootModel, BlockRootModelWithId},
    committee::{CommitteeModel, CommitteeModelsWithId},
    persistable::ResolvablePersistable,
    utils::{ModelCache, PersistableCache},
    vote::VoteModel,
};

use crate::{db::Stores, types::consolidated_block::ConsolidatedBlock};

pub fn spawn_persist_block_worker<E: EthSpec>(
    base_dir: String,
    stores: Arc<Stores<E>>,
    mut shutdown_trigger: Receiver<()>,
    executor: &TaskExecutor,
) -> UnboundedSender<ConsolidatedBlock<E>> {
    let (new_block_send, mut new_block_recv) = unbounded_channel();

    let mut extended_blocks_cache = ModelCache::new(base_dir.clone());
    let mut votes_cache = PersistableCache::new(base_dir.clone());

    executor.spawn(
        async move {
            loop {
                tokio::select! {
                    Some(block) = new_block_recv.recv() => {
                        if let Err(err) = persist_block::<E>(
                            &base_dir, block,
                            stores.block_roots_cache(),
                            stores.committees_cache(),
                            &mut extended_blocks_cache,
                            &mut votes_cache)
                        {
                            error!("{err}");
                        }
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

#[instrument(level = "debug", name = "BlockPersist", fields(duration), skip_all)]
fn persist_block<E: EthSpec>(
    base_dir: &str,
    block: ConsolidatedBlock<E>,
    block_roots_cache: Arc<RwLock<ModelCache<BlockRootModel>>>,
    committees_cache: Arc<RwLock<ModelCache<Vec<CommitteeModel>>>>,
    extended_blocks_cache: &mut ModelCache<Option<BlockExtendedModel>>,
    votes_cache: &mut PersistableCache<Vec<VoteModel>>,
) -> Result<(), String> {
    debug!(slot = %block.slot(), "Persisting block");
    let mut block_roots_cache = block_roots_cache.write();
    let mut committees_cache = committees_cache.write();

    BlockModelWithId::from(&block).save(base_dir).unwrap();
    BlockExtendedModelWithId::from(&block)
        .save(base_dir)
        .unwrap();
    AttestationModelsWithId::from(&block)
        .save(base_dir)
        .unwrap();
    CommitteeModelsWithId::from(&block).save(base_dir).unwrap();
    Option::<BlockRootModelWithId>::from(&block)
        .save(base_dir)
        .unwrap();

    block.attestations().iter().try_for_each(|attestation| {
        if let Ok(m) =
            block_roots_cache.get_mut(format!("{:?}", attestation.data.beacon_block_root))
        {
            let slot = attestation.data.slot.as_u64();

            votes_cache
                .get_or_default_mut(m.model.slot)
                .model
                .push(VoteModel {
                    slot,
                    included_in: block.slot().as_u64(),
                    committee_index: attestation.data.index,
                    validators: get_attesting_indices::<E>(
                        &committees_cache
                            .get_mut(slot)?
                            .model
                            .get(attestation.data.index as usize)
                            .ok_or(format!(
                                "The attestation '{}' is out of bound of committees at slot {slot}",
                                attestation.data.index
                            ))?
                            .validators,
                        &attestation.aggregation_bits,
                    )
                    .map_err(|err| {
                        format!(
                            "Error while getting attesting indices on slot {} for committee index{}: {err:?}",
                            block.slot(),
                            attestation.data.index
                        )
                    })
                    .unwrap_or_default(),
                });
        }

        Ok::<_, String>(())
    })?;

    votes_cache.dirty_iter().for_each(|votes| {
        if let Err(err) = extended_blocks_cache.update_and_persist(votes.id, |block_extended| {
            if let Some(model) = &mut block_extended.model {
                model.votes_count = votes.model.iter().map(|v| v.validators.len()).sum()
            }
        }) {
            error!(
                "Unable to update votes for extended block '{}': {err}",
                votes.id
            );
        }
    });

    votes_cache.persist_dirty();

    BlocksMeta::new(block.slot().as_usize() + 1)
        .save(base_dir)
        .unwrap();

    Ok(())
}
