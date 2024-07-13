use solana_client::{client_error::reqwest::Client, rpc_client::RpcClient};
use thiserror::Error;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
const PUMP_FUN_TRADE_API_URL: &'static str = "https://pumpapi.fun/api/trade_transaction";

mod input;

pub struct PumpTransactionHandler<'a> {
    api: Client,
    rpc: &'a RpcClient,
}

#[derive(Debug, Error)]
pub enum PumpTransactionHandlerError {
    #[error("ApiClientError: {0}")]
    BuildError(String),
    #[error("InternalError: {0}")]
    InternalError(String),
}

impl<'a> PumpTransactionHandler<'a> {
    fn new(rpc: &'a RpcClient) -> Result<Self, PumpTransactionHandlerError> {
        let api = Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()
            .map_err(|err| PumpTransactionHandlerError::BuildError(err.to_string()))?;

        Ok(Self { rpc, api })
    }

    async fn send_buy_call(&self) -> Result<(), PumpTransactionHandlerError> {
        // let resp = self.api.get(PUMP_FUN_TRADE_API_URL)
        Ok(())
    }
}
