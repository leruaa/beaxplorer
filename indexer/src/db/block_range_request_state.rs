use lighthouse_network::PeerId;

#[derive(Debug, Clone, Default, Copy)]
pub enum BlockRangeRequestState {
    #[default]
    Idle,
    AwaitingPeer,
    Requesting(PeerId),
}

impl BlockRangeRequestState {
    pub fn is_requesting(&self) -> bool {
        matches!(self, Self::Requesting(_))
    }

    pub fn matches(&self, peer_id: &PeerId) -> bool {
        match self {
            Self::Requesting(requesting_peer_id) => requesting_peer_id == peer_id,
            _ => false,
        }
    }

    pub fn set_to_requesting(&mut self, peer_id: PeerId) {
        *self = Self::Requesting(peer_id)
    }

    pub fn set_to_awaiting_peer(&mut self) {
        *self = Self::AwaitingPeer
    }
}
