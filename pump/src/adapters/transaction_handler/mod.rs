use mana_core::{
    entities::transaction_order::TransactionOrder,
    traits::transaction_handler::{TransactionHandler, TransactionHandlerResult},
    value_objects::transaction::transaction_type::TradeTransactionType,
};
use solana_client::{client_error::reqwest::header::HeaderMap, rpc_client::RpcClient};
use solana_sdk::{signature::Signature, transaction::Transaction};
use thiserror::Error;

use crate::pump_client::{PumpApiClient, PumpApiClientError};

pub struct PumpTransactionHandler<'a> {
    api: &'a PumpApiClient,
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

impl From<PumpApiClientError> for PumpTransactionHandlerError {
    fn from(error: PumpApiClientError) -> Self {
        PumpTransactionHandlerError::ApiClientError(error.to_string())
    }
}

impl<'a> PumpTransactionHandler<'a> {
    pub fn new(
        rpc: &'a RpcClient,
        api: &'a PumpApiClient,
    ) -> Result<Self, PumpTransactionHandlerError> {
        let mut headers = HeaderMap::new();

        headers.insert("Content-Type", "application/json".parse().unwrap());

        Ok(Self { rpc, api })
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
            .api
            .trade_transaction(TradeTransactionType::Buy, order)
            .await?;
        let mut tx: Transaction = tx_str
            .try_into()
            .map_err(PumpTransactionHandlerError::InvalidTransaction)?;

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
            .api
            .trade_transaction(TradeTransactionType::Sell, order)
            .await?;
        let mut tx: Transaction = tx_str
            .try_into()
            .map_err(PumpTransactionHandlerError::InvalidTransaction)?;

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
