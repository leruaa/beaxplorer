use std::{collections::HashMap, fmt::Debug, net::SocketAddr, sync::Arc};

use lighthouse_network::{multiaddr::multiaddr, Multiaddr, NetworkGlobals, PeerDB, PeerId};
use lighthouse_types::EthSpec;
use parking_lot::RwLockReadGuard;
use tracing::{error, info};
use types::good_peer::{GoodPeerModel, GoodPeerModelWithId};

pub struct PeerDb<E: EthSpec> {
    network_globals: Arc<NetworkGlobals<E>>,
    good_peers: HashMap<PeerId, Multiaddr>,
}

impl<E: EthSpec> PeerDb<E> {
    pub fn new(
        network_globals: Arc<NetworkGlobals<E>>,
        good_peers: HashMap<PeerId, Multiaddr>,
    ) -> Self {
        PeerDb {
            network_globals,
            good_peers,
        }
    }

    pub fn is_good_peer(&self, peer_id: &PeerId) -> bool {
        self.good_peers.contains_key(peer_id)
    }

    pub fn add_good_peer(&mut self, id: PeerId) {
        let peer_db = self.network_globals.peers.read();

        if let Some(info) = peer_db.peer_info(&id) {
            let addr = info.seen_addresses().next().map(|a| match a {
                SocketAddr::V4(a) => multiaddr!(Ip4(*a.ip()), Tcp(a.port())),
                SocketAddr::V6(a) => multiaddr!(Ip6(*a.ip()), Tcp(a.port())),
            });

            if let Some(addr) = addr {
                if self.good_peers.insert(id, addr).is_none() {
                    info!(peer = %id, "New good peer");
                }
            } else {
                error!(peer = %id, "The peer address can't be found");
            }
        } else {
            error!(peer = %id, "The peer's info isn't known");
        }
    }

    pub fn get_best_connected_peer(&self) -> Option<PeerId> {
        self.network_globals
            .peers
            .read()
            .best_by_status(|p| p.is_connected() && p.enr().is_some())
            .cloned()
    }

    pub fn good_peers_iter<'a>(
        &'a self,
    ) -> GoodPeerIterator<'a, E, impl Iterator<Item = &'a PeerId>> {
        GoodPeerIterator::new(
            self.good_peers.iter().map(|(id, _)| id),
            self.network_globals.peers.read(),
        )
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
            .map(|(id, addr)| GoodPeerModelWithId {
                id: id.to_string(),
                model: GoodPeerModel {
                    address: addr.to_string(),
                },
            })
            .collect()
    }
}
