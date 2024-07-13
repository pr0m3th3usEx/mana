use mana_core::entities::transaction_order::TransactionOrder;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all_fields(serialize = "lowercase"))]
pub enum TradeTransactionType {
    Buy,
    Sell,
}

impl ToString for TradeTransactionType {
    fn to_string(&self) -> String {
        match self {
            TradeTransactionType::Buy => "buy".to_string(),
            TradeTransactionType::Sell => "sell".to_string(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeTransactionBody {
    #[serde(rename = "trade_type")]
    trade_type: String,
    mint: String,
    amount: f64,
    slippage: u8,
    priority_fee: f64,
    user_public_key: String,
}

impl From<(TradeTransactionType, TransactionOrder)> for TradeTransactionBody {
    fn from((trade_type, order): (TradeTransactionType, TransactionOrder)) -> Self {
        Self {
            trade_type: trade_type.to_string(),
            amount: order.amount.value(),
            mint: order.mint.value().to_string(),
            priority_fee: order.priority_fee.value(),
            slippage: (order.slippage_tolerance.value() * 100f64) as u8,
            user_public_key: order.transactor.value().to_string(),
        }
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

    use crate::adapters::transaction_handler::input::TradeTransactionType;

    use super::TradeTransactionBody;

    fn fixture_order() -> TransactionOrder {
        let mint = TokenAddress::new("9fgRqJAAgibo889jN6wPvnVCuE5KLHwoFm9xqRfZ3cG6")
            .expect("valid pubkey");
        let transactor =
            Transactor::new("FvPatBa8aX9UrrPVdyHeib4sNWxuQWQne98RmEe74vYJ").expect("valid pubkey");
        let slippage = SlippageTolerance::new(0.25).unwrap();
        let amount = Amount::new(1.0, 9).unwrap();
        let priority_fee = PriorityFee::new(0.00001).unwrap();

        TransactionOrder::new(mint, transactor, amount, slippage, priority_fee)
    }

    #[test]
    fn test_from() {
        let tx = fixture_order();
        let input = TradeTransactionBody::from((TradeTransactionType::Buy, tx));

        assert_eq!(input.trade_type, "buy");
        assert_eq!(input.amount, 1.0);
        assert_eq!(input.mint, "9fgRqJAAgibo889jN6wPvnVCuE5KLHwoFm9xqRfZ3cG6");
        assert_eq!(
            input.user_public_key,
            "FvPatBa8aX9UrrPVdyHeib4sNWxuQWQne98RmEe74vYJ"
        );
        assert_eq!(input.slippage, 25);
        assert_eq!(input.priority_fee, 0.00001);
    }

    #[test]
    fn test_serialize_trade_tx() {
        let tx = fixture_order();
        let input = TradeTransactionBody::from((TradeTransactionType::Buy, tx));

        let body_str = serde_json::to_string(&input);

        assert!(body_str.is_ok());

        let body = body_str.unwrap();

        assert!(body.contains("buy"))
    }
}
