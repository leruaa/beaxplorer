use types::epoch::EpochModelWithId;

use crate::orderable::Orderable;

pub trait PersistableField {
    type Model;
    type Field: Ord + Eq + Send + Clone;
    const FIELD_NAME: &'static str;

    fn get_value(model: &Self::Model) -> Orderable<Self::Field>;
}

pub struct EpochAttestationsCount;

impl PersistableField for EpochAttestationsCount {
    type Model = EpochModelWithId;
    type Field = usize;
    const FIELD_NAME: &'static str = "attestations_count";

    fn get_value(model: &Self::Model) -> Orderable<Self::Field> {
        (model.0, model.1.attestations_count).into()
    }
}

pub struct EpochDepositsCount;

impl PersistableField for EpochDepositsCount {
    type Model = EpochModelWithId;
    type Field = usize;
    const FIELD_NAME: &'static str = "deposits_count";

    fn get_value(model: &Self::Model) -> Orderable<Self::Field> {
        (model.0, model.1.deposits_count).into()
    }
}
