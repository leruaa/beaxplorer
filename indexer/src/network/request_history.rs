use beacon_node::beacon_chain::parking_lot::RwLock;
use libp2p::PeerId;
use lighthouse_network::{rpc::RequestId, Request};

#[derive(Debug)]
pub struct RequestHistory {
    pub id: RwLock<Option<(RequestId, PeerId)>>,
    pub request: Request,
    pub not_found_on: RwLock<Vec<PeerId>>,
}

impl RequestHistory {
    pub fn eq(&self, request_id: RequestId) -> bool {
        self.id.read().map(|x| x.0).eq(&Some(request_id))
    }

    pub fn is_sent_to(&self, peer_id: PeerId) -> bool {
        self.id.read().map(|x| x.1).eq(&Some(peer_id))
    }

    pub fn is_pending(&self) -> bool {
        self.id.read().is_none()
    }

    pub fn reset_id(&self) {
        *self.id.write() = None
    }

    pub fn get_or_insert_id_with<F>(&self, f: F) -> (RequestId, PeerId)
    where
        F: FnOnce() -> (RequestId, PeerId),
    {
        *self.id.write().get_or_insert_with(f)
    }

    pub fn as_request_with_id(&self, id: &mut usize, peer_id: PeerId) -> (RequestId, Request) {
        let request_id = RequestId::Sync(*id);
        *self.id.write() = Some((request_id, peer_id));
        *id += 1;
        (request_id, self.request.clone())
    }

    pub fn was_not_found_on(&self, peer_id: &PeerId) -> bool {
        self.not_found_on.read().contains(peer_id)
    }

    pub fn mark_as_failed(&self) {
        let mut id = self.id.write();
        if let Some(id) = *id {
            self.not_found_on.write().push(id.1)
        }

        *id = None
    }
}

impl From<Request> for RequestHistory {
    fn from(request: Request) -> Self {
        RequestHistory {
            id: RwLock::new(None),
            request,
            not_found_on: RwLock::new(Vec::new()),
        }
    }
}

impl From<&RequestHistory> for Request {
    fn from(request_history: &RequestHistory) -> Self {
        request_history.request.clone()
    }
}
