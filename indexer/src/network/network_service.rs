use std::sync::Arc;

use beacon_node::ClientConfig;
use environment::RuntimeContext;
use eth2_network_config::Eth2NetworkConfig;

use lighthouse_network::{
    rpc::RequestId, Context, Libp2pEvent, NetworkGlobals, PeerId, Request, Service,
};
use slog::Logger;
use store::{BeaconState, ForkContext, MainnetEthSpec};

pub struct NetworkService {
    network_globals: Arc<NetworkGlobals<MainnetEthSpec>>,
    service: Service<MainnetEthSpec>,
    log: Logger,
}

impl NetworkService {
    pub async fn new(
        context: RuntimeContext<MainnetEthSpec>,
        config: ClientConfig,
        eth2_network_config: Eth2NetworkConfig,
    ) -> Result<Self, String> {
        let spec = context.eth2_config().spec.clone();
        let genesis_state_bytes = eth2_network_config.genesis_state_bytes.unwrap();
        let genesis_state =
            BeaconState::<MainnetEthSpec>::from_ssz_bytes(&genesis_state_bytes, &spec)
                .map_err(|e| format!("Unable to parse genesis state SSZ: {:?}", e))?;
        let enr_fork_id = spec.enr_fork_id::<MainnetEthSpec>(
            spec.genesis_slot,
            genesis_state.genesis_validators_root(),
        );
        let fork_context = Arc::new(ForkContext::new::<MainnetEthSpec>(
            spec.genesis_slot,
            genesis_state.genesis_validators_root(),
            &spec,
        ));
        let executor = context.clone().executor;
        let network_log = executor.log().clone();

        let service_context = Context {
            config: &config.network,
            enr_fork_id,
            fork_context: fork_context.clone(),
            chain_spec: &spec,
            gossipsub_registry: None,
        };

        let (network_globals, service) =
            Service::<MainnetEthSpec>::new(context.executor.clone(), service_context, &network_log)
                .await
                .unwrap();

        let network_service = Self {
            network_globals,
            service,
            log: network_log,
        };

        Ok(network_service)
    }

    pub async fn send_request(&mut self, peer_id: PeerId, request_id: RequestId, request: Request) {
        self.service.send_request(peer_id, request_id, request)
    }

    pub async fn next_event(&mut self) -> Libp2pEvent<MainnetEthSpec> {
        self.service.next_event().await
    }
}
