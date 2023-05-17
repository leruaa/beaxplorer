mod consensus_network;
mod execution_network;

pub use consensus_network::{
    spawn as spawn_consensus_network, NetworkCommand as ConsensusNetworkCommand, RequestId,
};

pub use execution_network::{
    spawn as spawn_execution_network, NetworkCommand as ExecutionNetworkCommand,
    NetworkEvent as ExecutionNetworkEvent,
};
