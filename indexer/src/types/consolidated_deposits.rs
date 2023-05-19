use lighthouse_types::EthSpec;
use parking_lot::RwLockWriteGuard;
use types::{
    deposit::{ConsensusLayerDepositModel, ConsensusLayerDepositModelWithId},
    persistable::ResolvablePersistable,
    utils::MetaCache,
};

use super::consolidated_block::ConsolidatedBlock;

pub struct ConsolidatedDeposits<'a> {
    deposits: Vec<ConsensusLayerDepositModel>,
    meta_cache: RwLockWriteGuard<'a, MetaCache>,
}

impl<'a> ConsolidatedDeposits<'a> {
    pub fn save(mut self, base_path: &str) -> Result<(), String> {
        let init_count = self.meta_cache.count::<ConsensusLayerDepositModel>();

        let saved_count = self
            .deposits
            .into_iter()
            .enumerate()
            .map(|(i, d)| ConsensusLayerDepositModelWithId {
                id: (init_count + i) as u64,
                model: d,
            })
            .try_fold(0_usize, |acc, deposit| {
                deposit
                    .save(base_path)
                    .map(|_| acc + 1)
                    .map_err(|err| (err, acc))
            });

        match saved_count {
            Ok(count) => {
                self.meta_cache
                    .update_and_save::<ConsensusLayerDepositModel, _>(|m| m.count += count)?;

                Ok(())
            }
            Err((err, count)) => {
                self.meta_cache
                    .update_and_save::<ConsensusLayerDepositModel, _>(|m| m.count += count)?;

                Err(err)
            }
        }
    }
}

impl<'a, E: EthSpec> From<(&ConsolidatedBlock<E>, RwLockWriteGuard<'a, MetaCache>)>
    for ConsolidatedDeposits<'a>
{
    fn from((block, meta_cache): (&ConsolidatedBlock<E>, RwLockWriteGuard<'a, MetaCache>)) -> Self {
        let deposits = Vec::<ConsensusLayerDepositModel>::from(block);

        Self {
            deposits,
            meta_cache,
        }
    }
}
