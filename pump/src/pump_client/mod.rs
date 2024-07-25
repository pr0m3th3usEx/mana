use std::time::Duration;

use input::TradeTransactionRequest;
use isahc::{
    config::{Configurable, RedirectPolicy},
    AsyncReadResponseExt, HttpClient,
};
use mana_core::{
    entities::transaction_order::TransactionOrder,
    value_objects::{
        token::token_address::TokenAddress, transaction::transaction_type::TradeTransactionType,
    },
};
use output::TradeTransactionResponse;
use solana_sdk::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;
use thiserror::Error;

use crate::{
    value_objects::token_liquidity::PumpTokenLiquidity, PUMP_BONDING_CURVE_SEED,
    PUMP_PROGRAM_ADDRESS,
};

pub mod input;
pub mod output;

static PUMP_API_TRADE_TRANSACTION_ENDPOINT: &str =
    concat!("https://pumpapi.fun/api", "/trade_transaction");

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(Debug, Error)]
pub enum PumpApiClientError {
    #[error("Bad request: {0}")]
    BadRequestError(String),

    #[error("DeserializationError: {0}")]
    DeserializationError(String),

    #[error("Unknow error: {0}")]
    UnknownError(String),

    #[error("Client error: {0}")]
    ClientError(String),
}

impl From<isahc::Error> for PumpApiClientError {
    fn from(error: isahc::Error) -> Self {
        Self::ClientError(error.to_string())
    }
}

pub struct PumpApiClient {
    http_client: HttpClient,
}

impl PumpApiClient {
    pub fn new() -> Result<Self, PumpApiClientError> {
        let http_client = HttpClient::builder()
            .timeout(Duration::from_secs(10))
            .redirect_policy(RedirectPolicy::None)
            .default_header("Content-Type", "application/json")
            .default_header("User-Agent", APP_USER_AGENT)
            .build()?;

        Ok(Self { http_client })
    }

    pub async fn trade_transaction(
        &self,
        trade_type: TradeTransactionType,
        order: TransactionOrder,
    ) -> Result<TradeTransactionResponse, PumpApiClientError> {
        // Serialize request body
        let req_body = serde_json::to_string(&TradeTransactionRequest::from((trade_type, order)))
            .map_err(|err| PumpApiClientError::BadRequestError(err.to_string()))?;

        // Send Pump.fun API request
        let mut response = self
            .http_client
            .post_async(PUMP_API_TRADE_TRANSACTION_ENDPOINT, req_body)
            .await
            .map_err(|err| PumpApiClientError::UnknownError(err.to_string()))?;

        response
            .json::<TradeTransactionResponse>()
            .await
            .map_err(|err| PumpApiClientError::DeserializationError(err.to_string()))
    }

    pub async fn get_token_liquidity(
        &self,
        mint: &TokenAddress,
    ) -> Result<PumpTokenLiquidity, PumpApiClient> {
        let (bonding_curve_owner, seed) = Pubkey::find_program_address(
            &[PUMP_BONDING_CURVE_SEED.as_bytes(), &mint.value().to_bytes()],
            &PUMP_PROGRAM_ADDRESS,
        );

        let bonding_curve_token_account =
            get_associated_token_address(&bonding_curve_owner, &mint.value());

        println!(
            "owner: {} - seed: {} - bounding curve: {}",
            bonding_curve_owner, seed, bonding_curve_token_account
        );

        Ok(PumpTokenLiquidity::new(1_000_000_000f64, 20.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mana_core::{
        entities::transaction_order::TransactionOrder,
        value_objects::{
            priority_fee::PriorityFee,
            slippage_tolerance::SlippageTolerance,
            token::token_address::TokenAddress,
            transaction::{
                amount::Amount, transaction_type::TradeTransactionType, transactor::Transactor,
            },
        },
        LAMPORTS_DECIMALS,
    };
    use solana_sdk::signature::Keypair;

    #[tokio::test]
    async fn test_should_retrieve_serialized_transaction_buy() {
        let api_client = PumpApiClient::new().unwrap();

        let order = TransactionOrder::new(
            TokenAddress::new("7dHz3cbenrGBL8iJ5fkPyae9so4uNceYzaYSEAu3pump").unwrap(),
            Transactor::new(Keypair::new()).unwrap(),
            Amount::new(0.0001, LAMPORTS_DECIMALS).unwrap(),
            SlippageTolerance::new(0.2).unwrap(),
            PriorityFee::new(0.000001).unwrap(),
        );

        let result = api_client
            .trade_transaction(TradeTransactionType::Buy, order)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_should_retrieve_serialized_transaction_sell() {
        let api_client = PumpApiClient::new().unwrap();

        let order = TransactionOrder::new(
            TokenAddress::new("7dHz3cbenrGBL8iJ5fkPyae9so4uNceYzaYSEAu3pump").unwrap(),
            Transactor::new(Keypair::new()).unwrap(),
            Amount::new(0.0001, LAMPORTS_DECIMALS).unwrap(),
            SlippageTolerance::new(0.2).unwrap(),
            PriorityFee::new(0.000001).unwrap(),
        );

        let result = api_client
            .trade_transaction(TradeTransactionType::Buy, order)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_liquidity() {
        let api_client = PumpApiClient::new().unwrap();
        let mint = TokenAddress::new("3mzfgTgcfDgvGVZFjtsqQcjuGRpWaBUH6nZ6jk9Dpump").unwrap();

        let result = api_client.get_token_liquidity(&mint).await;
        assert!(result.is_ok());
    }
}
