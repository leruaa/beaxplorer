use std::time::Duration;

use eth2::{
    lighthouse::{GlobalValidatorInclusionData, Peer},
    types::{
        BlockId, CommitteeData, ForkVersionedResponse, GenericResponse, ProposerData, RootData,
        StateId, ValidatorBalanceData, ValidatorData,
    },
    BeaconNodeHttpClient, Timeouts,
};
use futures::Future;
use lighthouse_types::{Epoch, EthSpec, SignedBeaconBlock};
use sensitive_url::SensitiveUrl;

use crate::errors::IndexerError;

#[derive(Clone)]
pub struct BeaconNodeClient {
    client: BeaconNodeHttpClient,
}

impl BeaconNodeClient {
    pub fn new(endpoint_url: String) -> Self {
        let url = SensitiveUrl::parse(&endpoint_url).unwrap();

        BeaconNodeClient {
            client: BeaconNodeHttpClient::new(url, Timeouts::set_all(Duration::from_secs(60))),
        }
    }

    pub fn get_block<E: EthSpec>(
        &self,
        block: BlockId,
    ) -> impl Future<Output = Result<Option<ForkVersionedResponse<SignedBeaconBlock<E>>>, IndexerError>>
    {
        let client = self.client.clone();

        async move {
            client
                .get_beacon_blocks::<E>(block)
                .await
                .map_err(|inner_error| IndexerError::NodeError { inner_error })
        }
    }

    pub fn get_block_root(
        &self,
        block: BlockId,
    ) -> impl Future<Output = Result<GenericResponse<RootData>, IndexerError>> {
        let client = self.client.clone();

        async move {
            client
                .get_beacon_blocks_root(block)
                .await
                .map_err(|inner_error| IndexerError::NodeError { inner_error })?
                .ok_or_else(|| IndexerError::ElementNotFound(block.to_string()))
        }
    }

    pub fn get_validators(
        &self,
        state: StateId,
    ) -> impl Future<Output = Result<Vec<ValidatorData>, IndexerError>> {
        let client = self.client.clone();

        async move {
            client
                .get_beacon_states_validators(state, None, None)
                .await
                .transpose()
                .ok_or_else(|| IndexerError::ElementNotFound(state.to_string()))?
                .map(|response| response.data)
                .map_err(|inner_error| IndexerError::NodeError { inner_error })
        }
    }

    pub fn get_validators_balances(
        &self,
        state: StateId,
    ) -> impl Future<Output = Result<Vec<ValidatorBalanceData>, IndexerError>> {
        let client = self.client.clone();

        async move {
            client
                .get_beacon_states_validator_balances(state, None)
                .await
                .transpose()
                .ok_or_else(|| IndexerError::ElementNotFound(state.to_string()))?
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

    pub fn get_committees(
        &self,
        epoch: Epoch,
    ) -> impl Future<Output = Result<Vec<CommitteeData>, IndexerError>> {
        let client = self.client.clone();

        async move {
            client
                .get_beacon_states_committees(StateId::Head, None, None, Some(epoch))
                .await
                .transpose()
                .ok_or_else(|| IndexerError::ElementNotFound(epoch.to_string()))?
                .map(|response| response.data)
                .map_err(|inner_error| IndexerError::NodeError { inner_error })
        }
    }

    pub fn get_peers<E: EthSpec>(
        &self,
    ) -> impl Future<Output = Result<Vec<Peer<E>>, IndexerError>> {
        let client = self.client.clone();

        async move {
            client
                .get_lighthouse_peers::<E>()
                .await
                .map_err(|inner_error| IndexerError::NodeError { inner_error })
        }
    }

    pub fn get_connected_peers<E: EthSpec>(
        &self,
    ) -> impl Future<Output = Result<Vec<Peer<E>>, IndexerError>> {
        let client = self.client.clone();

        async move {
            client
                .get_lighthouse_connected_peers::<E>()
                .await
                .map_err(|inner_error| IndexerError::NodeError { inner_error })
        }
    }
}
