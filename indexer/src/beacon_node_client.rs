use eth2::{
    lighthouse::GlobalValidatorInclusionData,
    types::{
        BlockId, GenericResponse, ProposerData, RootData, StateId, ValidatorBalanceData,
        ValidatorData,
    },
    BeaconNodeHttpClient,
};
use futures::Future;
use sensitive_url::SensitiveUrl;
use types::{Epoch, EthSpec, SignedBeaconBlock, Slot};

use crate::errors::IndexerError;

pub struct BeaconNodeClient {
    client: BeaconNodeHttpClient,
}

impl BeaconNodeClient {
    pub fn new(endpoint_url: String) -> Self {
        let url = SensitiveUrl::parse(&endpoint_url).unwrap();

        BeaconNodeClient {
            client: BeaconNodeHttpClient::new(url),
        }
    }

    pub fn get_block<E: EthSpec>(
        &self,
        slot: Slot,
    ) -> impl Future<Output = Result<Option<GenericResponse<SignedBeaconBlock<E>>>, IndexerError>>
    {
        let client = self.client.clone();

        async move {
            client
                .get_beacon_blocks::<E>(BlockId::Slot(slot))
                .await
                .map_err(|inner_error| IndexerError::NodeError { inner_error })
        }
    }

    pub fn get_block_root(
        &self,
        slot: Slot,
    ) -> impl Future<Output = Result<GenericResponse<RootData>, IndexerError>> {
        let client = self.client.clone();

        async move {
            client
                .get_beacon_blocks_root(BlockId::Slot(slot))
                .await
                .map_err(|inner_error| IndexerError::NodeError { inner_error })?
                .ok_or(IndexerError::ElementNotFound(slot))
        }
    }

    pub fn get_validators(
        &self,
        slot: Slot,
    ) -> impl Future<Output = Result<Vec<ValidatorData>, IndexerError>> {
        let client = self.client.clone();

        async move {
            client
                .get_beacon_states_validators(StateId::Slot(slot), None, None)
                .await
                .transpose()
                .ok_or(IndexerError::ElementNotFound(slot))?
                .map(|response| response.data)
                .map_err(|inner_error| IndexerError::NodeError { inner_error })
        }
    }

    pub fn get_validators_balances(
        &self,
        slot: Slot,
    ) -> impl Future<Output = Result<Vec<ValidatorBalanceData>, IndexerError>> {
        let client = self.client.clone();

        async move {
            client
                .get_beacon_states_validator_balances(StateId::Slot(slot), None)
                .await
                .transpose()
                .ok_or(IndexerError::ElementNotFound(slot))?
                .map(|response| response.data)
                .map_err(|inner_error| IndexerError::NodeError { inner_error })
        }
    }

    pub fn get_validator_inclusion(
        &self,
        epoch: Epoch,
    ) -> impl Future<Output = Result<GlobalValidatorInclusionData, IndexerError>> {
        let client = self.client.clone();

        async move {
            client
                .get_lighthouse_validator_inclusion_global(epoch)
                .await
                .map(|response| response.data)
                .map_err(|inner_error| IndexerError::NodeError { inner_error })
        }
    }

    pub fn get_validator_duties_proposer(
        &self,
        epoch: Epoch,
    ) -> impl Future<Output = Result<Vec<ProposerData>, IndexerError>> {
        let client = self.client.clone();

        async move {
            client
                .get_validator_duties_proposer(epoch)
                .await
                .map(|response| response.data)
                .map_err(|inner_error| IndexerError::NodeError { inner_error })
        }
    }
}
