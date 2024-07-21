use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
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
