use std::{collections::HashMap, sync::Arc};

use super::network_service::NetworkService;
use eth2::lighthouse::Peer;
use libp2p::{Multiaddr, PeerId};
use rand::{seq::IteratorRandom, thread_rng};
use store::MainnetEthSpec;
use tokio::sync::Mutex;

type PeerMap<'a> = HashMap<PeerId, &'a Multiaddr>;

pub struct PeersManager<'a> {
    elligible_peers: PeerMap<'a>,
    connected_peers: PeerMap<'a>,
}

impl<'a> PeersManager<'a> {
    pub async fn new(
        peers: &'a [Peer<MainnetEthSpec>],
        network_manager: Arc<Mutex<NetworkService>>,
    ) -> PeersManager<'a> {
        let mut rng = thread_rng();
        let elligible_peers = peers
            .iter()
            .map(|p| {
                (
                    p.peer_id.parse::<PeerId>().unwrap(),
                    p.peer_info.listening_addresses().first().unwrap(),
                )
            })
            .collect::<PeerMap>();

        let peers_to_connect = elligible_peers
            .iter()
            .map(|(p, _)| *p)
            .choose_multiple(&mut rng, 10);

        let (elligible_peers, peers_to_connect) = elligible_peers
            .into_iter()
            .partition::<PeerMap, _>(|(p, _)| peers_to_connect.contains(p));

        for (peer_id, multiaddr) in &peers_to_connect {
            network_manager
                .lock()
                .await
                .connect(*peer_id, multiaddr)
                .await;
        }

        PeersManager {
            elligible_peers,
            connected_peers: peers_to_connect,
        }
    }
}
