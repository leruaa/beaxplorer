use std::{collections::HashSet, sync::Arc};

use lighthouse_network::{Multiaddr, NetworkGlobals, PeerId, PeerInfo};
use slog::{info, Logger};
use store::EthSpec;

#[derive(Clone)]
pub struct PeerDb<E: EthSpec> {
    network_globals: Arc<NetworkGlobals<E>>,
    great_peers: HashSet<PeerId>,
    log: Logger,
}

type PeerTupleVec<E> = Vec<(PeerId, PeerInfo<E>)>;

impl<E: EthSpec> PeerDb<E> {
    pub fn new(network_globals: Arc<NetworkGlobals<E>>, log: Logger) -> Self {
        PeerDb {
            network_globals,
            great_peers: HashSet::new(),
            log,
        }
    }

    pub fn get_great_peers_known_addresses(&self) -> Vec<Multiaddr> {
        vec![
            "/ip4/51.79.202.73/tcp/13000".parse().unwrap(),
            "/ip4/76.141.229.155/tcp/13000".parse().unwrap(),
            "/ip4/178.128.188.228/tcp/13000".parse().unwrap(),
            "/ip4/107.184.229.134/tcp/13000".parse().unwrap(),
            "/ip4/76.69.229.226/tcp/13000".parse().unwrap(),
            "/ip4/67.174.112.67/tcp/13000".parse().unwrap(),
            "/ip4/8.9.30.14/tcp/13000".parse().unwrap(),
            "/ip4/173.174.120.56/tcp/13000".parse().unwrap(),
            "/ip4/34.230.190.149/tcp/13000".parse().unwrap(),
            "/ip4/98.0.57.197/tcp/13103".parse().unwrap(),
            "/ip4/98.13.141.186/tcp/13103".parse().unwrap(),
            "/ip4/204.13.164.143/tcp/13000".parse().unwrap(),
            "/ip4/104.186.143.194/tcp/13000".parse().unwrap(),
            "/ip4/54.65.63.75/tcp/13000".parse().unwrap(),
            "/ip4/15.164.101.121/tcp/13000".parse().unwrap(),
            "/ip4/121.78.247.249/tcp/13000".parse().unwrap(),
            "/ip4/209.151.145.125/tcp/13000".parse().unwrap(),
            "/ip4/95.111.198.189/tcp/13000".parse().unwrap(),
            "/ip4/66.42.64.100/tcp/13000".parse().unwrap(),
            "/ip4/139.9.74.98/tcp/13000".parse().unwrap(),
            "/ip4/178.128.13.206/tcp/13000".parse().unwrap(),
            "/ip4/99.130.254.231/tcp/13000".parse().unwrap(),
            "/ip4/76.93.16.249/tcp/13000".parse().unwrap(),
        ]
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

    pub fn add_great_peer(&mut self, peer_id: PeerId) {
        info!(self.log, "New great peer: {peer_id}");
        self.great_peers.insert(peer_id);
    }

    pub fn is_known_great_peer(&self, peer_id: &PeerId) -> bool {
        let great_peers_known_addresses = self.get_great_peers_known_addresses();

        if let Some(peer_info) = self.network_globals.peers.read().peer_info(peer_id) {
            for a in peer_info.listening_addresses() {
                if great_peers_known_addresses.contains(a) {
                    return true;
                }
            }
            false
        } else {
            false
        }
    }

    pub fn get_great_peers(&self) -> (PeerTupleVec<E>, PeerTupleVec<E>) {
        let peer_db = self.network_globals.peers.read();
        self.great_peers
            .iter()
            .filter_map(|peer_id| peer_db.peer_info(peer_id).map(|p| (*peer_id, p.clone())))
            .partition(|(_, p)| p.is_connected())
    }

    pub fn get_trusted_peers(&self) -> (PeerTupleVec<E>, PeerTupleVec<E>) {
        self.network_globals
            .peers
            .read()
            .best_peers_by_status(|p| p.is_trusted())
            .iter()
            .map(|(peer_id, peer_info)| (**peer_id, peer_info.to_owned().clone()))
            .partition(|(_, p)| p.is_connected())
    }
}
