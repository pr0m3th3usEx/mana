use std::error::Error;

use thiserror::Error;

use crate::entities::transaction_order::TransactionOrder;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Error)]
pub enum TransactionHandlerError {
    #[error("Internal Error: {0}")]
    InternalError(String),
}

pub trait TransactionHandler {
    /// Args:
    /// - order: [`TransactionOrder`]
    ///
    /// Returns:
    /// Confirmed transaction or [`TransactionHandlerError`]
    async fn buy(&self, order: TransactionOrder) -> Result<()>;

    async fn sell(&self, order: TransactionOrder) -> Result<()>;
}
