use std::{sync::Arc, time::Duration};

use crate::beacon_chain::beacon_context::BeaconContext;
use eth2::lighthouse::DepositLog;
use execution_layer::HttpJsonRpc;
use lighthouse_types::{ChainSpec, EthSpec};
use sensitive_url::SensitiveUrl;
use task_executor::TaskExecutor;
use tracing::{error, info};

/// Timeout when doing an eth_getLogs to read the deposit contract logs.
const GET_DEPOSIT_LOG_TIMEOUT_MILLIS: u64 = 60_000;

pub struct ExecutionService {
    endpoint: HttpJsonRpc,
    spec: ChainSpec,
    deposit_contract_address: String,
    deposit_contract_deploy_block: u64,
}

impl ExecutionService {
    pub async fn start<E: EthSpec>(
        executor: TaskExecutor,
        execution_node_url: SensitiveUrl,
        beacon_context: Arc<BeaconContext<E>>,
    ) -> Result<(), String> {
        let execution_service = Self {
            endpoint: HttpJsonRpc::new(execution_node_url, None)
                .map_err(|err| format!("{err:?}"))?,
            spec: beacon_context.spec.clone(),
            deposit_contract_address: format!("{:?}", beacon_context.spec.deposit_contract_address),
            deposit_contract_deploy_block: beacon_context
                .eth2_network_config
                .deposit_contract_deploy_block,
        };

        execution_service.spawn(executor);

        Ok(())
    }

    pub fn spawn(self, executor: TaskExecutor) {
        executor.spawn(
            async move {
                let mut current_deposit_block = self.deposit_contract_deploy_block;

                loop {
                    match self.get_deposits(current_deposit_block).await {
                        Ok(logs) => {
                            info!(
                                block = current_deposit_block,
                                count = logs.len(),
                                "Deposit logs query succeeded"
                            );
                            current_deposit_block += 1000;
                        }
                        Err(err) => {
                            error!(err)
                        }
                    }
                }
            },
            "execution network",
        );
    }

    pub async fn get_deposits(&self, block: u64) -> Result<Vec<DepositLog>, String> {
        self.endpoint
            .get_deposit_logs_in_range(
                &self.deposit_contract_address,
                block..block + 1000,
                Duration::from_millis(GET_DEPOSIT_LOG_TIMEOUT_MILLIS),
            )
            .await?
            .into_iter()
            .map(|raw_log| raw_log.to_deposit_log(&self.spec))
            .collect()
    }
}
