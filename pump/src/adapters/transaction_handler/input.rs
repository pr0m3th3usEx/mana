use std::fmt::Display;

use mana_core::entities::transaction_order::TransactionOrder;
use serde::{Deserialize, Serialize};
use solana_sdk::{
    signer::Signer,
    transaction::{Transaction, VersionedTransaction},
};

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all_fields(serialize = "lowercase"))]
pub enum TradeTransactionType {
    Buy,
    Sell,
}

impl Display for TradeTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeTransactionType::Buy => write!(f, "buy"),
            TradeTransactionType::Sell => write!(f, "sell"),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeTransactionRequest {
    #[serde(rename = "trade_type")]
    trade_type: String,
    mint: String,
    amount: f64,
    slippage: u8,
    priority_fee: f64,
    user_public_key: String,
}

impl From<(TradeTransactionType, TransactionOrder)> for TradeTransactionRequest {
    fn from((trade_type, order): (TradeTransactionType, TransactionOrder)) -> Self {
        Self {
            trade_type: trade_type.to_string(),
            amount: order.amount.value(),
            mint: order.mint.value().to_string(),
            priority_fee: order.priority_fee.value(),
            slippage: (order.slippage_tolerance.value() * 100f64) as u8,
            user_public_key: order.transactor.value().pubkey().to_string(),
        }
    }
}

#[derive(Deserialize)]
pub struct TradeTransactionResponse {
    pub transaction: String,
}

impl TryInto<Transaction> for TradeTransactionResponse {
    type Error = String;

    // Convert into unsigned transaction
    fn try_into(self) -> Result<Transaction, Self::Error> {
        let tx_bytes = bs58::decode(self.transaction).into_vec().map_err(|err| {
            // TODO log error
            err.to_string()
        })?;
        let versioned_tx: VersionedTransaction =
            bincode::deserialize(&tx_bytes).map_err(|err| {
                // TODO log error
                err.to_string()
            })?;

        versioned_tx
            .into_legacy_transaction()
            .ok_or("Could not convert into legacy transaction".to_string())
    }
}

#[cfg(test)]
mod tests {
    use mana_core::{
        entities::transaction_order::TransactionOrder,
        value_objects::{
            priority_fee::PriorityFee,
            slippage_tolerance::SlippageTolerance,
            token::token_address::TokenAddress,
            transaction::{amount::Amount, transactor::Transactor},
        },
    };
    use solana_sdk::{signature::Keypair, signer::Signer};

    use crate::adapters::transaction_handler::input::TradeTransactionType;

    use super::TradeTransactionRequest;

    fn fixture_order(keypair: Keypair) -> TransactionOrder {
        let mint = TokenAddress::new("9fgRqJAAgibo889jN6wPvnVCuE5KLHwoFm9xqRfZ3cG6")
            .expect("valid pubkey");
        let transactor = Transactor::new(keypair).expect("valid keypair");
        let slippage = SlippageTolerance::new(0.25).unwrap();
        let amount = Amount::new(1.0, 9).unwrap();
        let priority_fee = PriorityFee::new(0.00001).unwrap();

        TransactionOrder::new(mint, transactor, amount, slippage, priority_fee)
    }

    #[test]
    fn test_from() {
        let keypair = Keypair::new();
        let tx = fixture_order(keypair.insecure_clone());
        let input = TradeTransactionRequest::from((TradeTransactionType::Buy, tx));

        assert_eq!(input.trade_type, "buy");
        assert_eq!(input.amount, 1.0);
        assert_eq!(input.mint, "9fgRqJAAgibo889jN6wPvnVCuE5KLHwoFm9xqRfZ3cG6");
        assert_eq!(input.user_public_key, keypair.pubkey().to_string());
        assert_eq!(input.slippage, 25);
        assert_eq!(input.priority_fee, 0.00001);
    }

    #[test]
    fn test_serialize_trade_tx() {
        let keypair = Keypair::new();
        let tx = fixture_order(keypair);
        let input = TradeTransactionRequest::from((TradeTransactionType::Buy, tx));

        let body_str = serde_json::to_string(&input);

        assert!(body_str.is_ok());

        let body = body_str.unwrap();

        assert!(body.contains("buy"))
    }
}
