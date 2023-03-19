use std::{collections::HashSet, net::SocketAddr, sync::Arc};

use lighthouse_network::{NetworkGlobals, PeerId, PeerInfo};
use multiaddr::multiaddr;
use parking_lot::RwLock;
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
    pub fn new(
        network_globals: Arc<NetworkGlobals<E>>,
        good_peers: HashSet<PeerId>,
        log: Logger,
    ) -> Self {
        PeerDb {
            network_globals,
            good_peers: RwLock::new(good_peers),
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

    pub fn get_good_peers(&self) -> PeerTupleVec<E> {
        let peer_db = self.network_globals.peers.read();
        self.good_peers
            .read()
            .iter()
            .filter_map(|peer_id| peer_db.peer_info(peer_id).map(|p| (*peer_id, p.clone())))
            .collect::<Vec<_>>()
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
            .get_good_peers()
            .iter()
            .map(|(id, info)| GoodPeerModelWithId {
                id: id.to_string(),
                model: GoodPeerModel {
                    address: info
                        .seen_addresses()
                        .next()
                        .map_or(String::default(), |a| match a {
                            SocketAddr::V4(a) => {
                                multiaddr!(Ip4(*a.ip()), Tcp(a.port())).to_string()
                            }
                            SocketAddr::V6(a) => {
                                multiaddr!(Ip6(*a.ip()), Tcp(a.port())).to_string()
                            }
                        }),
                },
            })
            .collect()
    }
}
