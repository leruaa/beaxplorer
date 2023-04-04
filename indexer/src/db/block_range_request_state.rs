use lighthouse_network::PeerId;

#[derive(Debug, Default)]
pub struct BlockRangeRequest {
    state: BlockRangeRequestState,
    latest_nonce: u64,
}

#[derive(Debug, Clone, Default, Copy)]
pub enum BlockRangeRequestState {
    #[default]
    Idle,
    AwaitingPeer,
    Requesting(u64, PeerId),
}

impl BlockRangeRequest {
    pub fn is_requesting(&self) -> bool {
        matches!(self.state, BlockRangeRequestState::Requesting(_, _))
    }

    pub fn matches_nonce(&self, nonce: u64) -> bool {
        match self.state {
            BlockRangeRequestState::Requesting(n, _) => n == nonce,
            _ => false,
        }
    }

    pub fn matches_peer(&self, peer_id: PeerId) -> bool {
        match self.state {
            BlockRangeRequestState::Requesting(_, id) => id == peer_id,
            _ => false,
        }
    }

    pub fn increment_nonce(&mut self) -> u64 {
        self.latest_nonce += 1;
        self.latest_nonce
    }

    pub fn request_peer_if_possible(&mut self, nonce: u64, peer_id: PeerId) -> bool {
        match self.state {
            BlockRangeRequestState::Requesting(n, _) => n == nonce,
            BlockRangeRequestState::Idle | BlockRangeRequestState::AwaitingPeer => {
                self.state = BlockRangeRequestState::Requesting(nonce, peer_id);
                true
            }
        }
    }
    pub fn set_to_awaiting_peer(&mut self) {
        self.state = BlockRangeRequestState::AwaitingPeer
    }
}
