use mana_core::{
    entities::transaction_order::TransactionOrder,
    value_objects::{
        priority_fee::PriorityFee,
        slippage_tolerance::SlippageTolerance,
        token::token_address::TokenAddress,
        transaction::{amount::Amount, transactor::Transactor},
    },
    LAMPORTS_DECIMALS,
};
use pump::adapters::transaction_handler::{input::TradeTransactionType, PumpTransactionHandler};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, signature::Keypair};

const DEVNET_ENDPOINT: &'static str = "https://api.devnet.solana.com";

#[tokio::test]
async fn test_should_retrieve_serialized_transaction_buy() {
    let rpc = RpcClient::new_with_commitment(DEVNET_ENDPOINT, CommitmentConfig::confirmed());
    let handler = PumpTransactionHandler::new(&rpc).unwrap();

    let order = TransactionOrder::new(
        TokenAddress::new("7dHz3cbenrGBL8iJ5fkPyae9so4uNceYzaYSEAu3pump").unwrap(),
        Transactor::new(Keypair::new()).unwrap(),
        Amount::new(0.0001, LAMPORTS_DECIMALS).unwrap(),
        SlippageTolerance::new(0.2).unwrap(),
        PriorityFee::new(0.000001).unwrap(),
    );

    let result = handler
        .trade_transaction(TradeTransactionType::Buy, order)
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_should_retrieve_serialized_transaction_sell() {
    let rpc = RpcClient::new_with_commitment(DEVNET_ENDPOINT, CommitmentConfig::confirmed());
    let handler = PumpTransactionHandler::new(&rpc).unwrap();

    let order = TransactionOrder::new(
        TokenAddress::new("7dHz3cbenrGBL8iJ5fkPyae9so4uNceYzaYSEAu3pump").unwrap(),
        Transactor::new(Keypair::new()).unwrap(),
        Amount::new(0.0001, LAMPORTS_DECIMALS).unwrap(),
        SlippageTolerance::new(0.2).unwrap(),
        PriorityFee::new(0.000001).unwrap(),
    );

    let result = handler
        .trade_transaction(TradeTransactionType::Buy, order)
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_should_buy_n_tokens() {
    // TODO: need money
}

#[tokio::test]
async fn test_should_sell_n_tokens() {
    // TODO: need money
}
