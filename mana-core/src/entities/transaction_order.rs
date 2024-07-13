use crate::value_objects::{
    priority_fee::PriorityFee,
    slippage_tolerance::SlippageTolerance,
    token::token_address::TokenAddress,
    transaction::{amount::Amount, transactor::Transactor},
};

#[derive(Clone, Copy)]
pub struct TransactionOrder {
    pub mint: TokenAddress,
    pub transactor: Transactor,
    pub amount: Amount, //
    pub slippage_tolerance: SlippageTolerance,
    pub priority_fee: PriorityFee,
}

impl TransactionOrder {
    pub fn new(
        mint: TokenAddress,
        transactor: Transactor,
        amount: Amount,
        slippage_tolerance: SlippageTolerance,
        priority_fee: PriorityFee,
    ) -> Self {
        Self {
            mint,
            transactor,
            amount,
            slippage_tolerance,
            priority_fee,
        }
    }
}
