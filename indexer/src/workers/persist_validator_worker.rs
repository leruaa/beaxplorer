use std::sync::Arc;

use lighthouse_types::EthSpec;
use task_executor::TaskExecutor;
use tokio::sync::mpsc::{Sender, UnboundedSender, unbounded_channel};
use types::{deposit::{ExecutionLayerDepositModelWithId}, persistable::ResolvablePersistable};

use crate::{db::Stores, types::consolidated_execution_layer_deposit::ConsolidatedExecutionLayerDeposit};

#[derive(Debug, Clone)]
pub enum ValidatorEvent {
    NewDepositFromExecutionLayer(ConsolidatedExecutionLayerDeposit)
}

pub fn spawn_persist_validator_worker<E: EthSpec>(
    base_dir: String,
    stores: Arc<Stores<E>>,
    executor: &TaskExecutor,
    _: Sender<()>,
) -> UnboundedSender<ValidatorEvent>
{
    let (validator_event_send, mut validator_event_recv) = unbounded_channel();

    executor.spawn(
        async move {
            while let Some(event) = validator_event_recv.recv().await {
                match event {
                    ValidatorEvent::NewDepositFromExecutionLayer(consolidated_deposit) => {
                        ExecutionLayerDepositModelWithId::from(&consolidated_deposit).save(&base_dir).unwrap();

                    },
                }
            }
        },
        "persist validator worker",
    );


    validator_event_send
}