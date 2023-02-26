use std::ops::Div;

use indexer_macro::persistable_field;
use ordered_float::OrderedFloat;
use types::{block_request::BlockRequestModelWithId, epoch::EpochModelWithId};

use crate::orderable::Orderable;

pub trait PersistableField<M> {
    type Field: Ord + Eq + Send + Clone;
    const FIELD_NAME: &'static str;

    fn get_value(model: &M) -> Orderable<Self::Field>;
}

#[persistable_field(EpochModelWithId, attestations_count, usize)]
pub struct EpochAttestationsCount;

#[persistable_field(EpochModelWithId, deposits_count, usize)]
pub struct EpochDepositsCount;

#[persistable_field(EpochModelWithId, attester_slashings_count, usize)]
pub struct EpochAttesterSlashingsCount;

#[persistable_field(EpochModelWithId, proposer_slashings_count, usize)]
pub struct EpochProposerSlashingsCount;

#[persistable_field(EpochModelWithId, eligible_ether, u64)]
pub struct EpochEligibleEther;

#[persistable_field(EpochModelWithId, voted_ether, u64)]
pub struct EpochVotedEther;

pub struct EpochGlobalParticipationRate;

impl PersistableField<EpochModelWithId> for EpochGlobalParticipationRate {
    type Field = OrderedFloat<f64>;
    const FIELD_NAME: &'static str = "global_participation_rate";

    fn get_value(value: &EpochModelWithId) -> Orderable<Self::Field> {
        let global_participation_rate =
            (value.model.voted_ether as f64).div(value.model.eligible_ether as f64);
        (value.id, OrderedFloat(global_participation_rate)).into()
    }
}

#[persistable_field(BlockRequestModelWithId, root, String)]
pub struct BlockRootRequestRoot;
