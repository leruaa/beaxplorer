use std::{sync::Arc, time::Duration};

use beacon_node::beacon_chain::slot_clock::{SlotClock, SystemTimeSlotClock};
use environment::RuntimeContext;
use eth2_network_config::Eth2NetworkConfig;
use store::{BeaconState, ChainSpec, EnrForkId, EthSpec, Slot};

pub struct BeaconContext<E: EthSpec> {
    pub spec: ChainSpec,
    pub slot_clock: SystemTimeSlotClock,
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
        let spec = &context.eth2_config.spec;
        let slot_clock = SystemTimeSlotClock::new(
            spec.genesis_slot,
            Duration::from_secs(genesis_state.genesis_time()),
            Duration::from_secs(spec.seconds_per_slot),
        );

        let beacon_context = BeaconContext {
            spec: spec.clone(),
            slot_clock,
            genesis_state,
            eth2_network_config: eth2_network_config.clone(),
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
