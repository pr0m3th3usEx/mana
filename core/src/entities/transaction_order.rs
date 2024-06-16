use std::num::NonZeroU64;

pub struct TransactionOrder {
    pub from_token: String, // PubKey
    pub to_token: String,   // PubKey
    pub amount: NonZeroU64, // Lamports
    pub slippage_bps: NonZeroU64,
    pub gas_bps: NonZeroU64, // Lamports
}
