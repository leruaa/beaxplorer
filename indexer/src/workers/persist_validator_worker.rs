use std::{collections::HashSet, sync::Arc};

use lighthouse_types::EthSpec;
use task_executor::TaskExecutor;
use tokio::sync::mpsc::{unbounded_channel, Sender, UnboundedSender};
use tracing::error;
use types::{
    deposit::ExecutionLayerDepositModelWithId, persistable::ResolvablePersistable,
    validator::ValidatorExtendedModel,
};

use crate::{
    db::Stores,
    types::{
        consolidated_execution_layer_deposit::ConsolidatedExecutionLayerDeposit,
        consolidated_validator::ConsolidatedValidator,
    },
};

#[derive(Debug, Clone)]
pub enum ValidatorEvent {
    NewDepositFromExecutionLayer(ConsolidatedExecutionLayerDeposit),
}

pub fn spawn_persist_validator_worker<E: EthSpec>(
    base_dir: String,
    stores: Arc<Stores<E>>,
    executor: &TaskExecutor,
    _: Sender<()>,
) -> UnboundedSender<ValidatorEvent> {
    let (validator_event_send, mut validator_event_recv) = unbounded_channel();

    executor.spawn(
        async move {
            while let Some(event) = validator_event_recv.recv().await {
                match event {
                    ValidatorEvent::NewDepositFromExecutionLayer(consolidated_deposit) => {
                        let mut beacon_state = stores.beacon_state_mut();
                        let current_epoch = beacon_state.current_epoch();
                        let validator_index = consolidated_deposit.validator_index as usize;
                        let (validators, balances) = beacon_state.validators_and_balances_mut();

                        ExecutionLayerDepositModelWithId::from(&consolidated_deposit)
                            .save(&base_dir)
                            .unwrap();

                        if let Some((validator, balance)) = validators
                            .get(validator_index)
                            .and_then(|v| balances.get(validator_index).map(|b| (v, b)))
                        {
                            let consolidated_validator =
                                ConsolidatedValidator::<E>::new(validator, current_epoch, balance);

                            stores
                                .validators_cache()
                                .write()
                                .entry(consolidated_deposit.validator_index)
                                .update_or_insert(consolidated_validator.into())
                                .save(&base_dir)
                                .unwrap();

                            stores
                                .validators_extended_cache()
                                .write()
                                .entry(consolidated_deposit.validator_index)
                                .and_modify(|v| {
                                    v.model
                                        .execution_layer_deposits
                                        .insert(consolidated_deposit.index);
                                })
                                .or_insert_with(|| ValidatorExtendedModel {
                                    execution_layer_deposits: HashSet::from([
                                        consolidated_deposit.index
                                    ]),
                                })
                                .save(&base_dir)
                                .unwrap();
                        } else {
                            error!(
                                "The validator '{}' can't be found in the beacon state",
                                validator_index
                            );
                        }
                    }
                }
            }
        },
        "persist validator worker",
    );

    validator_event_send
}
