use std::time::Duration;

use beacon_node::beacon_chain::builder::{
    BeaconChainBuilder as InternalBeaconChainBuilder, Witness,
};
use beacon_node::beacon_chain::eth1_chain::DummyEth1ChainBackend;
use beacon_node::beacon_chain::slot_clock::{SlotClock, SystemTimeSlotClock};
use beacon_node::beacon_chain::BeaconChain;
use beacon_node::ClientConfig;
use slog::Logger;
use store::{EthSpec, HotColdDB, LevelDB};
use task_executor::TaskExecutor;

use super::beacon_context::BeaconContext;

pub type ConcreteWitness<E> =
    Witness<SystemTimeSlotClock, DummyEth1ChainBackend<E>, E, LevelDB<E>, LevelDB<E>>;

pub struct BeaconChainBuilder<'a, E: EthSpec> {
    context: Option<&'a BeaconContext<E>>,
    log: Option<&'a Logger>,
}

impl<'a, E: EthSpec> BeaconChainBuilder<'a, E> {
    pub fn new() -> Self {
        BeaconChainBuilder {
            context: None,
            log: None,
        }
    }

    pub fn logger(mut self, log: &'a Logger) -> Self {
        self.log = Some(log);
        self
    }

    pub fn context(mut self, context: &'a BeaconContext<E>) -> Self {
        self.context = Some(context);
        self
    }

    pub fn build(
        &self,
        executor: &TaskExecutor,
    ) -> Result<BeaconChain<ConcreteWitness<E>>, String> {
        let beacon_chain_builder = InternalBeaconChainBuilder::new(E::default());
        let context = self.context.ok_or("err")?;
        let log = self.log.ok_or("err")?;
        let client_config = ClientConfig::default();
        let store = HotColdDB::<E, LevelDB<E>, LevelDB<E>>::open(
            &client_config.create_data_dir()?,
            &client_config.create_freezer_db_path()?,
            |_, _, _| Ok(()),
            client_config.store,
            context.spec.clone(),
            log.clone(),
        )
        .map_err(|e| format!("Unable to open database: {:?}", e))?;
        let clock = SystemTimeSlotClock::new(
            context.spec.genesis_slot,
            Duration::from_secs(context.genesis_state.genesis_time()),
            Duration::from_secs(context.spec.seconds_per_slot),
        );

        beacon_chain_builder
            .logger(log.clone())
            .store(store)
            .genesis_state(context.genesis_state.clone())?
            .slot_clock(clock)
            .shutdown_sender(executor.shutdown_sender())
            .monitor_validators(false, vec![], log.clone())
            .eth1_backend(None as Option<DummyEth1ChainBackend<E>>)
            .build()
    }
}
