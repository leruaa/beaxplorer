use std::ops::Div;

use ordered_float::OrderedFloat;
use types::epoch::EpochModelWithId;

use crate::orderable::Orderable;

//use indexer_macro::persistable_field;

pub trait PersistableField<M> {
    type Field: Ord + Eq + Send + Clone;
    const FIELD_NAME: &'static str;

    fn get_value(model: &M) -> Orderable<Self::Field>;
}

//#[persistable_field(EpochModelWithId, attestations_count)]
pub struct EpochAttestationsCount;

impl PersistableField<EpochModelWithId> for EpochAttestationsCount {
    type Field = usize;
    const FIELD_NAME: &'static str = "attestations_count";

    fn get_value(value: &EpochModelWithId) -> Orderable<Self::Field> {
        (value.id, value.model.attestations_count).into()
    }
}

pub struct EpochDepositsCount;

impl PersistableField<EpochModelWithId> for EpochDepositsCount {
    type Field = usize;
    const FIELD_NAME: &'static str = "deposits_count";

    fn get_value(value: &EpochModelWithId) -> Orderable<Self::Field> {
        (value.id, value.model.deposits_count).into()
    }
}

pub struct EpochAttesterSlashingsCount;

impl PersistableField<EpochModelWithId> for EpochAttesterSlashingsCount {
    type Field = usize;
    const FIELD_NAME: &'static str = "attester_slashings_count";

    fn get_value(value: &EpochModelWithId) -> Orderable<Self::Field> {
        (value.id, value.model.attester_slashings_count).into()
    }
}

pub struct EpochProposerSlashingsCount;

impl PersistableField<EpochModelWithId> for EpochProposerSlashingsCount {
    type Field = usize;
    const FIELD_NAME: &'static str = "proposer_slashings_count";

    fn get_value(value: &EpochModelWithId) -> Orderable<Self::Field> {
        (value.id, value.model.proposer_slashings_count).into()
    }
}

pub struct EpochEligibleEther;

impl PersistableField<EpochModelWithId> for EpochEligibleEther {
    type Field = u64;
    const FIELD_NAME: &'static str = "eligible_ether";

    fn get_value(value: &EpochModelWithId) -> Orderable<Self::Field> {
        (value.id, value.model.eligible_ether).into()
    }
}

pub struct EpochVotedEther;

impl PersistableField<EpochModelWithId> for EpochVotedEther {
    type Field = u64;
    const FIELD_NAME: &'static str = "voted_ether";

    fn get_value(value: &EpochModelWithId) -> Orderable<Self::Field> {
        (value.id, value.model.voted_ether).into()
    }
}

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
