use std::sync::Arc;

use environment::RuntimeContext;
use eth2_network_config::Eth2NetworkConfig;
use store::{BeaconState, ChainSpec, EthSpec};

pub struct BeaconContext<E: EthSpec> {
    pub spec: ChainSpec,
    pub genesis_state: BeaconState<E>,
    pub eth2_network_config: Arc<Eth2NetworkConfig>,
}

impl<E: EthSpec> BeaconContext<E> {
    pub fn build(context: &RuntimeContext<E>) -> Result<Self, String> {
        let eth2_network_config = context
            .eth2_network_config
            .as_ref()
            .ok_or("The Eth2 network config is required")?;
        let genesis_state_bytes = eth2_network_config
            .genesis_state_bytes
            .as_ref()
            .ok_or("Genesis tate bytes are required")?;
        let genesis_state =
            BeaconState::from_ssz_bytes(genesis_state_bytes, &context.eth2_config.spec)
                .map_err(|_| "Failed to decode SSZ")?;

        let beacon_context = BeaconContext {
            spec: context.eth2_config.spec.clone(),
            genesis_state,
            eth2_network_config: eth2_network_config.clone(),
        };

        Ok(beacon_context)
    }
}