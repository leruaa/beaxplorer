use types::epoch::EpochModelWithId;

use crate::orderable::Orderable;

pub trait PersistableField {
    type Model;
    type Field: Ord + Eq + Send + Clone;

    fn get_field_name<'a>() -> &'a str;
    fn get_value(model: &Self::Model) -> Orderable<Self::Field>;
}

pub struct EpochDepositCount;

impl PersistableField for EpochDepositCount {
    type Model = EpochModelWithId;
    type Field = usize;

    fn get_field_name<'a>() -> &'a str {
        "deposits_count"
    }

    fn get_value(model: &Self::Model) -> Orderable<Self::Field> {
        (model.0, model.1.deposits_count).into()
    }
}
