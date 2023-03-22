use lighthouse_network::PeerId;
use lighthouse_types::Hash256;

#[derive(Debug, Clone)]
pub enum Work {
    Persist,
    SendRangeRequest(PeerId),
    SendBlockByRootRequest(PeerId, Hash256),
    SendNetworkMessage(Option<PeerId>),
}
