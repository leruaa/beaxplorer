use std::{collections::HashSet, sync::Arc};

use lighthouse_network::{NetworkGlobals, PeerId, PeerInfo};
use store::EthSpec;

#[derive(Clone)]
pub struct PeerDb<E: EthSpec> {
    network_globals: Arc<NetworkGlobals<E>>,
    great_peers: HashSet<PeerId>,
}

type PeerTupleVec<E> = Vec<(PeerId, PeerInfo<E>)>;

impl<E: EthSpec> PeerDb<E> {
    pub fn new(network_globals: Arc<NetworkGlobals<E>>) -> Self {
        PeerDb {
            network_globals,
            great_peers: HashSet::new(),
        }
    }

    pub fn get_best_connected_peer(&self) -> Option<PeerId> {
        self.network_globals
            .peers
            .read()
            .best_by_status(|p| p.is_connected() && p.enr().is_some())
            .cloned()
    }

    pub fn add_great_peer(&mut self, peer_id: PeerId) {
        self.great_peers.insert(peer_id);
    }

    pub fn get_great_peers(&self) -> (PeerTupleVec<E>, PeerTupleVec<E>) {
        let peer_db = self.network_globals.peers.read();
        self.great_peers
            .iter()
            .filter_map(|peer_id| peer_db.peer_info(peer_id).map(|p| (*peer_id, p.clone())))
            .partition(|(_, p)| p.is_connected())
    }
}
