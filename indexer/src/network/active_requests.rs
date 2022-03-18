use std::slice::Iter;

use lighthouse_network::rpc::RequestId;

use super::request_history::RequestHistory;

#[derive(Default)]
pub struct ActiveRequests(Vec<RequestHistory>);

impl ActiveRequests {
    pub fn find_by_request_id(&self, request_id: RequestId) -> Option<&RequestHistory> {
        self.0.iter().find(|x| x.eq(request_id))
    }

    pub fn push(&mut self, request_history: RequestHistory) {
        self.0.push(request_history)
    }

    pub fn remove(&mut self, request_id: RequestId) {
        self.0.retain(|r| !r.eq(request_id))
    }

    pub fn iter(&self) -> Iter<RequestHistory> {
        self.0.iter()
    }
}
