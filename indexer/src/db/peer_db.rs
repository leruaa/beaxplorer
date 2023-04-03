use std::{collections::HashSet, fmt::Debug, net::SocketAddr, sync::Arc};

use lighthouse_network::{NetworkGlobals, PeerDB, PeerId};
use lighthouse_types::EthSpec;
use multiaddr::multiaddr;
use parking_lot::RwLockReadGuard;
use std::collections::hash_set::Iter;
use tracing::info;
use types::good_peer::{GoodPeerModel, GoodPeerModelWithId};

pub struct PeerDb<E: EthSpec> {
    network_globals: Arc<NetworkGlobals<E>>,
    good_peers: HashSet<PeerId>,
}

impl<E: EthSpec> PeerDb<E> {
    pub fn new(network_globals: Arc<NetworkGlobals<E>>, good_peers: HashSet<PeerId>) -> Self {
        PeerDb {
            network_globals,
            good_peers,
        }
    }

    pub fn is_good_peer(&self, peer_id: &PeerId) -> bool {
        self.good_peers.contains(peer_id)
    }

    pub fn add_good_peer(&mut self, peer_id: PeerId) {
        if self.good_peers.insert(peer_id) {
            info!("New good peer: {peer_id}");
        }
    }

    pub fn get_best_connected_peer(&self) -> Option<PeerId> {
        self.network_globals
            .peers
            .read()
            .best_by_status(|p| p.is_connected() && p.enr().is_some())
            .cloned()
    }

    pub fn good_peers_iter<'a>(&'a self) -> GoodPeerIterator<'a, E, Iter<'a, PeerId>> {
        GoodPeerIterator::new(self.good_peers.iter(), self.network_globals.peers.read())
    }
}

impl<E: EthSpec> Debug for PeerDb<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.good_peers.fmt(f)
    }
}

pub struct GoodPeerIterator<'a, E: EthSpec, I> {
    iter: I,
    peer_db: Arc<RwLockReadGuard<'a, PeerDB<E>>>,
}

impl<'a, E: EthSpec, I> GoodPeerIterator<'a, E, I> {
    pub fn new(iter: I, peer_db: RwLockReadGuard<'a, PeerDB<E>>) -> Self {
        Self {
            iter,
            peer_db: Arc::new(peer_db),
        }
    }
}

impl<'a, E: EthSpec, I: Iterator<Item = &'a PeerId> + 'a> GoodPeerIterator<'a, E, I> {
    pub fn connected(self) -> impl Iterator<Item = &'a PeerId> {
        let peer_db = self.peer_db.clone();
        self.filter_map(move |id| {
            peer_db
                .peer_info(id)
                .and_then(|info| match info.is_connected() {
                    true => Some(id),
                    false => None,
                })
        })
    }
}

impl<'a, E: EthSpec, I: Iterator> Iterator for GoodPeerIterator<'a, E, I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<E: EthSpec> From<&PeerDb<E>> for Vec<GoodPeerModelWithId> {
    fn from(value: &PeerDb<E>) -> Self {
        let peer_db = value.network_globals.peers.read();

        value
            .good_peers
            .iter()
            .filter_map(|id| peer_db.peer_info(id).and_then(|info| Some((id, info))))
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
