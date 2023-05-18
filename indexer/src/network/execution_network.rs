use std::{ops::Range, sync::Arc, time::Duration};

use crate::beacon_chain::beacon_context::BeaconContext;
use eth2::lighthouse::DepositLog;
use execution_layer::HttpJsonRpc;
use lighthouse_types::{ChainSpec, EthSpec};
use sensitive_url::SensitiveUrl;
use task_executor::TaskExecutor;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tracing::{error, info};

/// Timeout when doing an eth_getLogs to read the deposit contract logs.
const GET_DEPOSIT_LOG_TIMEOUT_MILLIS: u64 = 60_000;

#[derive(Debug, Clone)]
pub enum NetworkCommand {
    RetrieveDeposits(Range<u64>),
}

#[derive(Debug, Clone)]
pub enum NetworkEvent {
    NewDeposits(Vec<DepositLog>),
}

pub struct ExecutionNetwork {
    endpoint: HttpJsonRpc,
    command_recv: UnboundedReceiver<NetworkCommand>,
    event_send: UnboundedSender<NetworkEvent>,
    spec: ChainSpec,
    deposit_contract_address: String,
}

pub fn spawn<E: EthSpec>(
    execution_node_url: SensitiveUrl,
    beacon_context: Arc<BeaconContext<E>>,
    executor: &TaskExecutor,
) -> Result<
    (
        UnboundedSender<NetworkCommand>,
        UnboundedReceiver<NetworkEvent>,
    ),
    String,
> {
    let (command_send, command_recv) = mpsc::unbounded_channel::<NetworkCommand>();
    let (event_send, event_recv) = mpsc::unbounded_channel::<NetworkEvent>();

    let execution_service = ExecutionNetwork {
        endpoint: HttpJsonRpc::new(execution_node_url, None).map_err(|err| format!("{err:?}"))?,
        command_recv,
        event_send,
        spec: beacon_context.spec.clone(),
        deposit_contract_address: format!("{:?}", beacon_context.spec.deposit_contract_address),
    };

    execution_service.spawn(executor);

    Ok((command_send, event_recv))
}

impl ExecutionNetwork {
    pub fn spawn(mut self, executor: &TaskExecutor) {
        executor.spawn(
            async move {

                while let Some(command) = self.command_recv.recv().await {
                    match command {
                        NetworkCommand::RetrieveDeposits(range) => {
                            match self.get_deposits(&range).await {
                                Ok(logs) => {
                                    info!(
                                        from = range.start,
                                        to = range.end,
                                        count = logs.len(),
                                        "Deposit logs query succeeded"
                                    );

                                    self.event_send.send(NetworkEvent::NewDeposits(logs)).unwrap();
                                }
                                Err(err) => {
                                    error!(err)
                                }
                            }
                        }
                    }
                }
            },
            "execution network",
        );
    }

    pub async fn get_deposits(&self, range: &Range<u64>) -> Result<Vec<DepositLog>, String> {
        self.endpoint
            .get_deposit_logs_in_range(
                &self.deposit_contract_address,
                range.clone(),
                Duration::from_millis(GET_DEPOSIT_LOG_TIMEOUT_MILLIS),
            )
            .await?
            .into_iter()
            .map(|raw_log| raw_log.to_deposit_log(&self.spec))
            .collect()
    }
}
