use crate::value_objects::{
    priority_fee::PriorityFee,
    slippage_tolerance::SlippageTolerance,
    token::token_address::TokenAddress,
    transaction::{amount::Amount, transactor::Transactor},
};

pub struct TransactionOrder {
    pub mint: TokenAddress,
    pub transactor: Transactor,
    pub amount: Amount, //
    pub slippage_tolerance: SlippageTolerance,
    pub priority_fee: PriorityFee,
}
