use std::sync::Arc;

use lighthouse_network::{
    rpc::{BlocksByRangeRequest, BlocksByRootRequest},
    Request,
};
use lighthouse_types::EthSpec;
use tokio::sync::mpsc::UnboundedSender;
use tracing::warn;
use types::{
    block_request::{BlockRequestModelWithId, BlockRequestsMeta},
    good_peer::{GoodPeerModelWithId, GoodPeersMeta},
    persistable::Persistable,
};

use crate::{
    db::{BlockByRootRequests, PeerDb, Stores},
    network::augmented_network_service::{NetworkCommand, RequestId},
    types::block_state::BlockState,
    work::Work,
};

pub fn handle<E: EthSpec>(
    base_dir: String,
    work: Work<E>,
    stores: &Arc<Stores<E>>,
    network_command_send: &UnboundedSender<NetworkCommand>,
    persist_work_send: &UnboundedSender<BlockState<E>>,
) {
    match work {
        Work::PersistBlock(block) => {
            persist_work_send.send(block).unwrap();
        }
        Work::PersistBlockRequest(root, attempts) => {
            let block_request = BlockRequestModelWithId::from((&root, &attempts));

            block_request.persist(&base_dir);
        }

        Work::PersistAllBlockRequests => {
            persist_block_requests(&base_dir, &stores.block_by_root_requests())
        }

        Work::PersistAllGoodPeers => persist_good_peers(&base_dir, &stores.peer_db()),

        Work::SendRangeRequest(to) => {
            let mut block_range_request = stores.block_range_request_mut();

            block_range_request.set_to_awaiting_peer();

            match to.or_else(|| stores.peer_db().get_best_connected_peer()) {
                Some(to) => {
                    let start_slot = stores
                        .latest_slot()
                        .map(|s| s.as_u64() + 1)
                        .unwrap_or_default();

                    network_command_send
                        .send(NetworkCommand::SendRequest {
                            peer_id: to,
                            request_id: RequestId::Range(block_range_request.increment_nonce()),
                            request: Box::new(Request::BlocksByRange(BlocksByRangeRequest {
                                start_slot,
                                count: 32,
                            })),
                        })
                        .unwrap();
                }
                None => {
                    warn!("No peer available to a new range request");
                }
            }
        }

        Work::SendBlockByRootRequest(root, to) => {
            network_command_send
                .send(NetworkCommand::SendRequest {
                    peer_id: to,
                    request_id: RequestId::Block(root),
                    request: Box::new(Request::BlocksByRoot(BlocksByRootRequest {
                        block_roots: vec![root].into(),
                    })),
                })
                .unwrap();
        }
    }
}

pub fn persist_block_requests(base_dir: &str, block_by_root_requests: &BlockByRootRequests) {
    let block_requests = Vec::<BlockRequestModelWithId>::from(block_by_root_requests);
    let meta = BlockRequestsMeta::new(block_requests.len());

    block_requests.persist(base_dir);
    meta.persist(base_dir);
}

pub fn persist_good_peers<E: EthSpec>(base_dir: &str, peer_db: &PeerDb<E>) {
    let good_peers = Vec::<GoodPeerModelWithId>::from(peer_db);
    let meta = GoodPeersMeta::new(good_peers.len());

    good_peers.persist(base_dir);
    meta.persist(base_dir);
    //info_span!(self.log, "Good peers persisted");
}
