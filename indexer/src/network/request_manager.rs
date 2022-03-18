use std::borrow::BorrowMut;

use libp2p::PeerId;
use lighthouse_network::{rpc::RequestId, BehaviourEvent, Libp2pEvent, Response};
use slog::{info, Logger};
use store::MainnetEthSpec;

use super::{
    active_requests::ActiveRequests, network_service::NetworkService,
    request_history::RequestHistory,
};

pub struct RequestManager {
    network_service: NetworkService,
    next_request_id: usize,
    active_requests: ActiveRequests,
    log: Logger,
}

impl RequestManager {
    pub fn new(network_service: NetworkService, log: Logger) -> Self {
        RequestManager {
            network_service,
            next_request_id: 0,
            active_requests: Default::default(),
            log,
        }
    }

    pub fn send_request<R: Into<RequestHistory>>(&mut self, request: R) {
        let request_history = request.into();
        let next_request_id = self.next_request_id.borrow_mut();
        self.network_service
            .send_request(&request_history, next_request_id);
        self.active_requests.push(request_history);
    }

    fn retry_request(&mut self, request_id: RequestId) {
        let active_request = self.active_requests.find_by_request_id(request_id);

        if let Some(active_request) = active_request {
            active_request.mark_as_failed();
            self.network_service
                .send_request(active_request, self.next_request_id.borrow_mut());
        }
    }

    fn retry_requests_by_peer_id(&mut self, peer_id: PeerId) {
        let network_service = self.network_service.borrow_mut();
        let next_request_id = self.next_request_id.borrow_mut();
        let log = self.log.borrow_mut();

        self.active_requests
            .iter()
            .filter(|r| r.is_sent_to(peer_id))
            .for_each(|r| {
                info!(log, "Retry request {:?}", r.id.read());
                r.mark_as_failed();
                network_service.send_request(r, next_request_id);
            })
    }

    fn send_pending_requests(&mut self, peer_id: &PeerId) {
        let next_request_id = self.next_request_id.borrow_mut();
        let pending_requests = self
            .active_requests
            .iter()
            .filter(|r| r.is_pending())
            .map(|r| r.as_request_with_id(next_request_id, *peer_id))
            .collect::<Vec<_>>();

        for r in pending_requests {
            info!(self.log, "Send pending request to {:?}", peer_id);
            self.network_service
                .send_request_to_peer(*peer_id, r.0, r.1);
        }
    }

    pub async fn next_event(&mut self) -> Response<MainnetEthSpec> {
        loop {
            if let Libp2pEvent::Behaviour(behaviour) = self.network_service.next_event().await {
                match behaviour {
                    BehaviourEvent::PeerConnectedOutgoing(peer_id) => {
                        self.send_pending_requests(&peer_id);
                    }
                    BehaviourEvent::RPCFailed { id, .. } => {
                        info!(self.log, "Retry request {:?}", id);
                        self.retry_request(id);
                    }
                    BehaviourEvent::PeerDisconnected(peer_id) => {
                        self.retry_requests_by_peer_id(peer_id);
                    }
                    BehaviourEvent::ResponseReceived { id, response, .. } => {
                        self.active_requests.remove(id);
                        return response;
                    }
                    _ => {}
                }
            }
        }
    }
}
