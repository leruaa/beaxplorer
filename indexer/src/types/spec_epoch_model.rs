use std::marker::PhantomData;

use db::models::EpochModel;
use lighthouse_types::EthSpec;

pub struct SpecEpochModel<'a, E: EthSpec> {
    pub inner: &'a EpochModel,
    phantom: PhantomData<E>,
}

impl<'a, E: EthSpec> SpecEpochModel<'a, E> {
    pub fn new(inner: &'a EpochModel) -> Self {
        SpecEpochModel {
            inner,
            phantom: PhantomData,
        }
    }
}
