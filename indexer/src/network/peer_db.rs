use std::{collections::HashSet, sync::Arc};

use lighthouse_network::{NetworkGlobals, PeerId, PeerInfo};
use parking_lot::{RwLock, RwLockReadGuard};
use slog::{info, Logger};
use store::EthSpec;
use types::good_peer::{GoodPeerModel, GoodPeerModelWithId};

pub struct PeerDb<E: EthSpec> {
    network_globals: Arc<NetworkGlobals<E>>,
    good_peers: RwLock<HashSet<PeerId>>,
    log: Logger,
}

type PeerTupleVec<E> = Vec<(PeerId, PeerInfo<E>)>;

impl<E: EthSpec> PeerDb<E> {
    pub fn new(network_globals: Arc<NetworkGlobals<E>>, log: Logger) -> Self {
        PeerDb {
            network_globals,
            good_peers: RwLock::new(HashSet::new()),
            log,
        }
    }

    pub fn get_peer_info(&self, peer_id: &PeerId) -> Option<PeerInfo<E>> {
        self.network_globals
            .peers
            .read()
            .peer_info(peer_id)
            .cloned()
    }

    pub fn get_best_connected_peer(&self) -> Option<PeerId> {
        self.network_globals
            .peers
            .read()
            .best_by_status(|p| p.is_connected() && p.enr().is_some())
            .cloned()
    }

    pub fn is_good_peer(&self, peer_id: &PeerId) -> bool {
        self.good_peers.read().contains(peer_id)
    }

    pub fn get_good_peers(&self) -> RwLockReadGuard<HashSet<PeerId>> {
        self.good_peers.read()
    }

    pub fn add_good_peer(&self, peer_id: PeerId) {
        info!(self.log, "New good peer: {peer_id}");
        self.good_peers.write().insert(peer_id);
    }

    pub fn get_connected_good_peers(&self) -> PeerTupleVec<E> {
        let peer_db = self.network_globals.peers.read();
        self.good_peers
            .read()
            .iter()
            .filter_map(|peer_id| peer_db.peer_info(peer_id).map(|p| (*peer_id, p.clone())))
            .filter(|(_, p)| p.is_connected())
            .collect::<Vec<_>>()
    }

    pub fn has_connected_good_peers(&self) -> bool {
        let peer_db = self.network_globals.peers.read();
        self.good_peers
            .read()
            .iter()
            .filter_map(|peer_id| peer_db.peer_info(peer_id).map(|p| (*peer_id, p.clone())))
            .filter(|(_, p)| p.is_connected())
            .count()
            > 0
    }
}

impl<E: EthSpec> From<&PeerDb<E>> for Vec<GoodPeerModelWithId> {
    fn from(value: &PeerDb<E>) -> Self {
        value
            .good_peers
            .read()
            .iter()
            .map(|p| GoodPeerModelWithId {
                id: p.to_string(),
                model: GoodPeerModel,
            })
            .collect()
    }
}
