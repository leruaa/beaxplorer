use std::time::Duration;

use beacon_node::beacon_chain::slot_clock::{SlotClock, SystemTimeSlotClock};
use environment::{Environment, EnvironmentBuilder};
use eth2_network_config::{Eth2NetworkConfig, DEFAULT_HARDCODED_NETWORK};
use store::{BeaconState, ChainSpec, EnrForkId, EthSpec, Slot};

#[derive(Clone)]
pub struct BeaconContext<E: EthSpec> {
    pub spec: ChainSpec,
    pub slot_clock: SystemTimeSlotClock,
    pub genesis_state: BeaconState<E>,
    pub eth2_network_config: Eth2NetworkConfig,
}

impl<E: EthSpec> BeaconContext<E> {
    pub fn build(spec: ChainSpec) -> Result<Self, String> {
        let eth2_network_config = Eth2NetworkConfig::constant(DEFAULT_HARDCODED_NETWORK)?
            .ok_or("Failed to build Eth2 network config")?;
        let genesis_state_bytes = eth2_network_config
            .genesis_state_bytes
            .as_ref()
            .ok_or("Genesis tate bytes are required")?;
        let genesis_state = BeaconState::from_ssz_bytes(genesis_state_bytes, &spec)
            .map_err(|_| "Failed to decode SSZ")?;
        let slot_clock = SystemTimeSlotClock::new(
            spec.genesis_slot,
            Duration::from_secs(genesis_state.genesis_time()),
            Duration::from_secs(spec.seconds_per_slot),
        );

        let beacon_context = BeaconContext {
            spec,
            slot_clock,
            genesis_state,
            eth2_network_config,
        };

        Ok(beacon_context)
    }

    pub fn new(genesis_state: BeaconState<E>, spec: ChainSpec) -> Result<Self, String> {
        let eth2_network_config = Eth2NetworkConfig::constant(DEFAULT_HARDCODED_NETWORK)?
            .ok_or("Failed to build Eth2 network config")?;

        let slot_clock = SystemTimeSlotClock::new(
            spec.genesis_slot,
            Duration::from_secs(genesis_state.genesis_time()),
            Duration::from_secs(spec.seconds_per_slot),
        );

        let beacon_context = BeaconContext {
            spec,
            slot_clock,
            genesis_state,
            eth2_network_config,
        };

        Ok(beacon_context)
    }

    pub fn current_slot(&self) -> Slot {
        self.slot_clock.now().unwrap_or(self.spec.genesis_slot)
    }

    pub fn current_fork_id(&self) -> EnrForkId {
        self.spec.enr_fork_id::<E>(
            self.current_slot(),
            self.genesis_state.genesis_validators_root(),
        )
    }
}

pub fn build_environment<E: EthSpec>(
    environment_builder: EnvironmentBuilder<E>,
) -> Result<(Environment<E>, Eth2NetworkConfig), String> {
    let eth2_network_config = Eth2NetworkConfig::constant(DEFAULT_HARDCODED_NETWORK)?
        .ok_or("Failed to build Eth2 network config")?;
    let environment = environment_builder
        .eth2_network_config(eth2_network_config.clone())?
        .null_logger()?
        .multi_threaded_tokio_runtime()?
        .build()?;

    Ok((environment, eth2_network_config))
}
