use std::sync::Arc;

use lighthouse_types::EthSpec;
use serde::Serialize;
use task_executor::TaskExecutor;
use tokio::sync::mpsc::UnboundedSender;

use tracing::info;
use types::{
    block_request::{BlockRequestModel, BlockRequestModelWithId},
    good_peer::{GoodPeerModel, GoodPeerModelWithId},
    persistable::{ResolvablePersistable},
};

use crate::{
    db::Stores, types::consolidated_block::ConsolidatedBlock, work::Work,
    workers::spawn_persist_epoch_worker,
};

pub fn handle<E: EthSpec + Serialize>(
    base_dir: String,
    work: Work<E>,
    stores: &Arc<Stores<E>>,
    new_block_send: &UnboundedSender<ConsolidatedBlock<E>>,
    executor: &TaskExecutor,
) {
    match work {
        Work::PersistIndexingState() => {
            persist_indexing_state(&base_dir, &stores);
        },

        },

        Work::PersistBlock(block) => new_block_send.send(block).unwrap(),

        Work::PersistEpoch(epoch) => spawn_persist_epoch_worker(base_dir, epoch, stores, executor),

        Work::PersistBlockRequest(root, attempts) => {
            let block_request = BlockRequestModelWithId::from((&root, &attempts));

            block_request.save(&base_dir).unwrap();
        }

        Work::PersistAllBlockRequests => persist_block_requests(&base_dir, stores),

        Work::PersistAllGoodPeers => persist_good_peers(&base_dir, stores),
    }
}

pub fn persist_indexing_state<E: EthSpec + Serialize>(base_dir: &str, stores: &Arc<Stores<E>>) {
    info!("Persisting indexing state");
    stores.indexing_state().save(&base_dir).unwrap();
}

pub fn persist_block_requests<E: EthSpec>(base_dir: &str, stores: &Arc<Stores<E>>) {
    let block_requests = Vec::<BlockRequestModelWithId>::from(&*stores.block_by_root_requests());

    block_requests.save(base_dir).unwrap();

    stores
        .meta_cache_mut()
        .update_and_save::<BlockRequestModel, _>(|m| m.count = block_requests.len())
        .unwrap();
}

pub fn persist_good_peers<E: EthSpec>(base_dir: &str, stores: &Arc<Stores<E>>) {
    let good_peers: Vec<GoodPeerModelWithId> = vec![]; //Vec::<GoodPeerModelWithId>::from(&*stores.peer_db());

    good_peers.save(base_dir).unwrap();

    stores
        .meta_cache_mut()
        .update_and_save::<GoodPeerModel, _>(|m| m.count = good_peers.len())
        .unwrap();
}
