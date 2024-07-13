use solana_sdk::signature::Signature;
use thiserror::Error;

use crate::entities::transaction_order::TransactionOrder;

pub type TransactionHandlerResult<T, E> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum TransactionHandlerError {
    #[error("Internal Error: {0}")]
    InternalError(String),
}

pub trait TransactionHandler {
    type Error;
    /// Args:
    /// - order: [`TransactionOrder`]
    ///
    /// Returns:
    /// Confirmed transaction [`Signature`] or [`Self::Error`]
    async fn buy(
        &self,
        order: TransactionOrder,
    ) -> TransactionHandlerResult<Signature, Self::Error>;

    /// Args:
    /// - order: [`TransactionOrder`]
    ///
    /// Returns:
    /// Confirmed transaction [`Signature`] or [`Self::Error`]
    async fn sell(
        &self,
        order: TransactionOrder,
    ) -> TransactionHandlerResult<Signature, Self::Error>;
}
