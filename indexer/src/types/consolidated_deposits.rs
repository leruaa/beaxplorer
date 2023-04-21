use lighthouse_types::EthSpec;
use parking_lot::RwLock;
use types::{
    deposit::{DepositModel, DepositModelWithId},
    persistable::ResolvablePersistable,
    utils::MetaCache,
};

use super::consolidated_block::ConsolidatedBlock;

pub struct ConsolidatedDeposits<'a> {
    deposits: Vec<DepositModel>,
    meta_cache: &'a RwLock<MetaCache>,
}

impl<'a> ConsolidatedDeposits<'a> {
    pub fn save(self, base_path: &str) -> Result<(), String> {
        let mut meta_cache = self.meta_cache.write();
        let init_count = meta_cache.count::<DepositModel>();

        let saved_count = self
            .deposits
            .into_iter()
            .enumerate()
            .map(|(i, d)| DepositModelWithId {
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
                meta_cache.update_and_save::<DepositModel, _>(|m| m.count += count)?;

                Ok(())
            }
            Err((err, count)) => {
                meta_cache.update_and_save::<DepositModel, _>(|m| m.count += count)?;

                Err(err)
            }
        }
    }
}

impl<'a, E: EthSpec> From<(&ConsolidatedBlock<E>, &'a RwLock<MetaCache>)>
    for ConsolidatedDeposits<'a>
{
    fn from((block, meta_cache): (&ConsolidatedBlock<E>, &'a RwLock<MetaCache>)) -> Self {
        let deposits = Vec::<DepositModel>::from(block);

        Self {
            deposits,
            meta_cache,
        }
    }
}
