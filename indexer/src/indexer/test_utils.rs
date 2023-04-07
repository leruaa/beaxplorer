use std::sync::Arc;

use beacon_node::beacon_chain::{
    builder::Witness, eth1_chain::CachingEth1Backend, slot_clock::ManualSlotClock,
    test_utils::BeaconChainHarness as LighthouseBeaconChainHarness,
};
use genesis::generate_deterministic_keypairs;
use lighthouse_network::NetworkGlobals;
use lighthouse_types::{
    BeaconBlock, BeaconState, EthSpec, MainnetEthSpec, Signature, SignedBeaconBlock, Slot,
};
use slog::{o, Logger};
use store::MemoryStore;
use tracing_slog::TracingSlogDrain;

use crate::db::Stores;

pub fn build_stores<E: EthSpec>() -> Arc<Stores<E>> {
    let logger = Logger::root(TracingSlogDrain, o!());
    let network_globals = NetworkGlobals::new_test_globals(&logger);

    Arc::new(Stores::new(Arc::new(network_globals), vec![], vec![]))
}

pub struct BeaconChainHarness {
    harness: LighthouseBeaconChainHarness<
        Witness<
            ManualSlotClock,
            CachingEth1Backend<MainnetEthSpec>,
            MainnetEthSpec,
            MemoryStore<MainnetEthSpec>,
            MemoryStore<MainnetEthSpec>,
        >,
    >,
    state: BeaconState<MainnetEthSpec>,
}

impl BeaconChainHarness {
    pub fn new() -> Self {
        let harness = LighthouseBeaconChainHarness::builder(MainnetEthSpec::default())
            .default_spec()
            .keypairs(generate_deterministic_keypairs(2))
            .fresh_ephemeral_store()
            .build();

        let state = harness.get_current_state();

        Self { harness, state }
    }

    pub async fn make_block(&mut self, slot: u64) -> SignedBeaconBlock<MainnetEthSpec> {
        if slot == 0 {
            SignedBeaconBlock::from_block(
                BeaconBlock::empty(&self.harness.spec),
                Signature::empty(),
            )
        } else {
            let (block, state) = self
                .harness
                .make_block(self.state.clone(), Slot::new(slot))
                .await;
            self.state = state;
            block
        }
    }
}
