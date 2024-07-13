use input::{TradeTransactionRequest, TradeTransactionResponse, TradeTransactionType};
use mana_core::{
    entities::transaction_order::TransactionOrder,
    traits::transaction_handler::{TransactionHandler, TransactionHandlerResult},
};
use solana_client::{
    client_error::reqwest::{header::HeaderMap, Client},
    rpc_client::RpcClient,
};
use solana_sdk::{signature::Signature, transaction::Transaction};
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
    #[error("BuildError: {0}")]
    BuildError(String),
    #[error("BodyError: {0}")]
    BodyError(String),
    #[error("InvalidTransaction: {0}")]
    InvalidTransaction(String),
    #[error("InvalidSignature: {0}")]
    InvalidSignature(String),
    #[error("RpcClientError: {0}")]
    RpcClientError(String),
    #[error("ApiClientError: {0}")]
    ApiClientError(String),
}

impl<'a> PumpTransactionHandler<'a> {
    pub fn new(rpc: &'a RpcClient) -> Result<Self, PumpTransactionHandlerError> {
        let mut headers = HeaderMap::new();

        headers.insert("Content-Type", "application/json".parse().unwrap());
        let api = Client::builder()
            .user_agent(APP_USER_AGENT)
            .default_headers(headers)
            .build()
            .map_err(|err| PumpTransactionHandlerError::BuildError(err.to_string()))?;

        Ok(Self { rpc, api })
    }

    async fn trade_transaction(
        &self,
        trade_type: TradeTransactionType,
        order: TransactionOrder,
    ) -> Result<TradeTransactionResponse, PumpTransactionHandlerError> {
        // Serialize request body
        let req_body = serde_json::to_string(&TradeTransactionRequest::from((trade_type, order)))
            .map_err(|err| {
            // TODO log error
            PumpTransactionHandlerError::BodyError(err.to_string())
        })?;

        // Send Pump.fun API request
        let response = self
            .api
            .post(PUMP_FUN_TRADE_API_URL)
            .body(req_body)
            .send()
            .await
            .map_err(|err| PumpTransactionHandlerError::ApiClientError(err.to_string()))?;

        // Deserialize response body
        let res_body = response
            .json::<TradeTransactionResponse>()
            .await
            .map_err(|err| PumpTransactionHandlerError::BodyError(err.to_string()))?;

        Ok(res_body)
    }
}

impl<'a> TransactionHandler for PumpTransactionHandler<'a> {
    type Error = PumpTransactionHandlerError;

    async fn buy(
        &self,
        order: TransactionOrder,
    ) -> TransactionHandlerResult<Signature, PumpTransactionHandlerError> {
        let keypair = order.transactor.value().insecure_clone();
        let tx_str = self
            .trade_transaction(TradeTransactionType::Buy, order)
            .await?;
        let mut tx: Transaction = tx_str
            .try_into()
            .map_err(|err| PumpTransactionHandlerError::InvalidTransaction(err))?;

        // Sign transaction

        let signers = vec![keypair];
        let recent_blockhash = self
            .rpc
            .get_latest_blockhash()
            .map_err(|err| PumpTransactionHandlerError::RpcClientError(err.to_string()))?;

        tx.try_sign(&signers, recent_blockhash)
            .map_err(|err| PumpTransactionHandlerError::InvalidSignature(err.to_string()))?;

        self.rpc
            .send_and_confirm_transaction(&tx)
            .map_err(|err| PumpTransactionHandlerError::RpcClientError(err.to_string()))
    }

    async fn sell(
        &self,
        order: TransactionOrder,
    ) -> TransactionHandlerResult<Signature, PumpTransactionHandlerError> {
        let keypair = order.transactor.value().insecure_clone();
        let tx_str = self
            .trade_transaction(TradeTransactionType::Sell, order)
            .await?;
        let mut tx: Transaction = tx_str
            .try_into()
            .map_err(|err| PumpTransactionHandlerError::InvalidTransaction(err))?;

        // Sign transaction

        let signers = vec![keypair];
        let recent_blockhash = self
            .rpc
            .get_latest_blockhash()
            .map_err(|err| PumpTransactionHandlerError::RpcClientError(err.to_string()))?;

        tx.try_sign(&signers, recent_blockhash)
            .map_err(|err| PumpTransactionHandlerError::InvalidSignature(err.to_string()))?;

        self.rpc
            .send_and_confirm_transaction(&tx)
            .map_err(|err| PumpTransactionHandlerError::RpcClientError(err.to_string()))
    }
}
