use std::{collections::HashMap, sync::Arc};

use beacon_node::ClientConfig;
use environment::RuntimeContext;
use eth2_network_config::Eth2NetworkConfig;

use lighthouse_network::{
    rpc::RequestId, BehaviourEvent, Context, Libp2pEvent, NetworkGlobals, PeerId, Request,
    Response, Service,
};
use slog::{info, Logger};
use std::hash::Hash;
use store::{BeaconState, ForkContext, MainnetEthSpec};
struct ActiveRequest {
    pub request: Request,
    pub not_found_on: Vec<PeerId>,
}

impl ActiveRequest {
    pub fn new(request: &Request) -> Self {
        ActiveRequest {
            request: request.clone(),
            not_found_on: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HashedRequestId(RequestId);

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for HashedRequestId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self.0 {
            RequestId::Router => "Router".hash(state),
            RequestId::Sync(id) => id.hash(state),
            RequestId::Behaviour => "Behaviour".hash(state),
        }
    }
}

impl From<RequestId> for HashedRequestId {
    fn from(request_id: RequestId) -> Self {
        HashedRequestId(request_id)
    }
}

pub struct NetworkService {
    next_request_id: usize,
    network_globals: Arc<NetworkGlobals<MainnetEthSpec>>,
    service: Service<MainnetEthSpec>,
    active_requests: HashMap<HashedRequestId, ActiveRequest>,
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
            next_request_id: 0,
            active_requests: HashMap::new(),
            network_globals,
            service,
            log: network_log,
        };

        Ok(network_service)
    }

    pub async fn init(&mut self) {
        let mut peer_count = 0;
        loop {
            if let Libp2pEvent::Behaviour(behaviour) = self.service.next_event().await {
                match behaviour {
                    BehaviourEvent::PeerConnectedOutgoing(_) => {
                        info!(self.log, "Peer added");
                        peer_count += 1
                    }
                    BehaviourEvent::PeerDisconnected(_) => {
                        info!(self.log, "Peer removed");
                        peer_count -= 1
                    }
                    _ => {}
                }
            }

            if peer_count >= 5 {
                info!(self.log, "Network service init completed");
                break;
            }
        }
    }

    pub fn send_request(&mut self, request: Request) -> Result<(), String> {
        let request_id = RequestId::Sync(self.next_request_id);
        let peers = self.network_globals.peers.read();
        let best_peer_id = peers
            .best_by_status(|p| p.is_connected())
            .ok_or("No connected peers found in DB")?;

        self.active_requests
            .insert(request_id.into(), ActiveRequest::new(&request));

        self.service
            .send_request(*best_peer_id, request_id, request);

        self.next_request_id += 1;

        Ok(())
    }

    fn retry_request(&mut self, request_id: RequestId, not_found: PeerId) -> Result<(), String> {
        let active_request = self
            .active_requests
            .get_mut(&request_id.into())
            .ok_or("No active requests")?;
        active_request.not_found_on.push(not_found);

        let peers = self.network_globals.peers.read();
        let best_peer_id = peers
            .best_peers_by_status(|p| p.is_connected())
            .iter()
            .map(|x| *x.0)
            .filter(|x| !active_request.not_found_on.contains(x))
            .collect::<Vec<_>>();

        let best_peer_id = best_peer_id
            .first()
            .ok_or("No connected peers found in DB")?;

        info!(self.log, "Retry {:?} on {:}", request_id, best_peer_id);

        self.service
            .send_request(*best_peer_id, request_id, active_request.request.clone());

        Ok(())
    }

    pub async fn next_event(&mut self) -> Response<MainnetEthSpec> {
        loop {
            if let Libp2pEvent::Behaviour(behaviour) = self.service.next_event().await {
                match behaviour {
                    BehaviourEvent::RPCFailed { id, peer_id } => {
                        self.retry_request(id, peer_id).unwrap();
                    }
                    BehaviourEvent::ResponseReceived {
                        peer_id,
                        id,
                        response,
                    } => return response,
                    _ => {}
                }
            }
        }
    }
}
