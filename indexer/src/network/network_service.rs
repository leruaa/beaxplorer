use std::sync::Arc;

use lighthouse_network::{
    rpc::RequestId, Context, Libp2pEvent, NetworkConfig, NetworkGlobals, PeerId, Request, Service,
};
use slog::{warn, Logger};
use store::{ForkContext, MainnetEthSpec};
use task_executor::TaskExecutor;

use crate::beacon_chain::beacon_context::BeaconContext;

use super::request_history::RequestHistory;

pub struct NetworkService {
    network_globals: Arc<NetworkGlobals<MainnetEthSpec>>,
    service: Service<MainnetEthSpec>,
    log: Logger,
}

impl NetworkService {
    pub async fn new(
        executor: TaskExecutor,
        beacon_context: &BeaconContext<MainnetEthSpec>,
    ) -> Result<Self, String> {
        let enr_fork_id = beacon_context.spec.enr_fork_id::<MainnetEthSpec>(
            beacon_context.spec.genesis_slot,
            beacon_context.genesis_state.genesis_validators_root(),
        );
        let fork_context = Arc::new(ForkContext::new::<MainnetEthSpec>(
            beacon_context.spec.genesis_slot,
            beacon_context.genesis_state.genesis_validators_root(),
            &beacon_context.spec,
        ));
        let network_log = executor.log().clone();
        let mut network = NetworkConfig::default();

        if let Some(boot_nodes) = &beacon_context.eth2_network_config.boot_enr {
            network.boot_nodes_enr.extend_from_slice(boot_nodes)
        }

        let service_context = Context {
            config: &network,
            enr_fork_id,
            fork_context: fork_context.clone(),
            chain_spec: &beacon_context.spec,
            gossipsub_registry: None,
        };

        let (network_globals, service) =
            Service::<MainnetEthSpec>::new(executor.clone(), service_context, &network_log)
                .await
                .unwrap();

        let network_service = Self {
            network_globals,
            service,
            log: network_log,
        };

        Ok(network_service)
    }

    pub fn send_request(&mut self, request_history: &RequestHistory, next_request_id: &mut usize) {
        let best_peers_id = self
            .network_globals
            .peers
            .read()
            .best_peers_by_status(|p| p.is_connected())
            .iter()
            .map(|x| *x.0)
            .filter(|x| !request_history.was_not_found_on(x))
            .collect::<Vec<_>>();

        let best_peer_id = best_peers_id.first();

        if let Some(peer_id) = best_peer_id {
            let request_id = request_history.get_or_insert_id_with(|| {
                *next_request_id += 1;
                (RequestId::Sync(*next_request_id - 1), *peer_id)
            });

            self.send_request_to_peer(*peer_id, request_id.0, request_history.into());
        } else {
            warn!(
                self.log,
                "No peers found to send request {:?}",
                request_history.id.read()
            );
            request_history.reset_id()
        }
    }

    pub fn send_request_to_peer(
        &mut self,
        peer_id: PeerId,
        request_id: RequestId,
        request: Request,
    ) {
        self.service.send_request(peer_id, request_id, request);
    }

    pub async fn next_event(&mut self) -> Libp2pEvent<MainnetEthSpec> {
        self.service.next_event().await
    }
}
