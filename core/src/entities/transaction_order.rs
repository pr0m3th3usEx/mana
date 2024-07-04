use std::num::NonZeroU64;

use crate::value_objects::{
    token::token_address::TokenAddress, transaction::transactor::Transactor,
};

pub struct TransactionOrder {
    pub mint: TokenAddress,
    pub transactor: Transactor,
    pub amount: NonZeroU64,       // Lamports
    pub slippage_bps: NonZeroU64, // Lamports
    pub gas_bps: NonZeroU64,      // Lamports
}
