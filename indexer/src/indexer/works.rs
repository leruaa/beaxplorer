use std::sync::Arc;

use lighthouse_network::{
    rpc::{BlocksByRangeRequest, BlocksByRootRequest},
    Request,
};
use lighthouse_types::EthSpec;
use task_executor::TaskExecutor;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{debug, warn};
use types::{
    block_request::{BlockRequestModel, BlockRequestModelWithId},
    good_peer::{GoodPeerModel, GoodPeerModelWithId},
    persistable::ResolvablePersistable,
};

use crate::{
    db::Stores,
    network::augmented_network_service::{NetworkCommand, RequestId},
    types::consolidated_block::ConsolidatedBlock,
    work::Work,
    workers::spawn_persist_epoch_worker,
};

pub fn handle<E: EthSpec>(
    base_dir: String,
    work: Work<E>,
    stores: &Arc<Stores<E>>,
    network_command_send: &UnboundedSender<NetworkCommand>,
    new_block_send: &UnboundedSender<ConsolidatedBlock<E>>,
    executor: &TaskExecutor,
) {
    match work {
        Work::PersistBlock(block) => new_block_send.send(block).unwrap(),

        Work::PersistEpoch(epoch) => spawn_persist_epoch_worker(base_dir, epoch, stores, executor),

        Work::PersistBlockRequest(root, attempts) => {
            let block_request = BlockRequestModelWithId::from((&root, &attempts));

            block_request.save(&base_dir).unwrap();
        }

        Work::PersistAllBlockRequests => persist_block_requests(&base_dir, stores),

        Work::PersistAllGoodPeers => persist_good_peers(&base_dir, stores),

        Work::SendRangeRequest(to) => {
            match to.or_else(|| stores.peer_db().get_best_connected_peer()) {
                Some(to) => {
                    let start_slot = stores
                        .indexing_state()
                        .latest_slot()
                        .map(|s| s.as_u64() + 1)
                        .unwrap_or_default();

                    debug!(start_slot, "Send range request");

                    network_command_send
                        .send(NetworkCommand::SendRequest {
                            peer_id: to,
                            request_id: RequestId::Range,
                            request: Box::new(Request::BlocksByRange(BlocksByRangeRequest {
                                start_slot,
                                count: 32,
                            })),
                        })
                        .unwrap();
                }
                None => {
                    warn!("No peer available for a new range request");
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

pub fn persist_block_requests<E: EthSpec>(base_dir: &str, stores: &Arc<Stores<E>>) {
    let block_requests = Vec::<BlockRequestModelWithId>::from(&*stores.block_by_root_requests());

    block_requests.save(base_dir).unwrap();

    stores
        .meta_cache()
        .write()
        .update_and_save::<BlockRequestModel, _>(|m| m.count = block_requests.len())
        .unwrap();
}

pub fn persist_good_peers<E: EthSpec>(base_dir: &str, stores: &Arc<Stores<E>>) {
    let good_peers = Vec::<GoodPeerModelWithId>::from(&*stores.peer_db());

    good_peers.save(base_dir).unwrap();

    stores
        .meta_cache()
        .write()
        .update_and_save::<GoodPeerModel, _>(|m| m.count = good_peers.len())
        .unwrap();
}
