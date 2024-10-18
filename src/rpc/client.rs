use anyhow::Context;
use web3::{transports::Http, types::SyncInfo, Web3};

use crate::types::HypermonError;

#[derive(Debug, Clone)]
pub struct RpcClient {
    client: Web3<Http>,
    pub rpc_url: String,
}

impl RpcClient {
    pub fn new(rpc_url: String) -> Result<RpcClient, HypermonError> {
        let transport = web3::transports::Http::new(rpc_url.as_str())
            .context("Unable to get web3 transport!")
            .map_err(|e| HypermonError::RpcClientError(e))?;
        let web3 = web3::Web3::new(transport);

        Ok(RpcClient {
            client: web3,
            rpc_url,
        })
    }

    pub async fn syncing_info(&self) -> Result<Option<SyncInfo>, HypermonError> {
        let res = self
            .client
            .eth()
            .syncing()
            .await
            .context(format!("Couldn't get syncing info for {}!", self.rpc_url))
            .map_err(|e| HypermonError::RpcClientError(e))?;

        match res {
            web3::types::SyncState::Syncing(sync_info) => Ok(Some(sync_info)),
            web3::types::SyncState::NotSyncing => Ok(None),
        }
    }

    pub async fn current_block(&self) -> Result<u64, HypermonError> {
        Ok(self
            .client
            .eth()
            .block_number()
            .await
            .context(format!(
                "Couldn't get current block number from {}!",
                self.rpc_url
            ))
            .map_err(|e| HypermonError::RpcClientError(e))?
            .to_string()
            .parse()
            .unwrap_or(0))
    }
}
