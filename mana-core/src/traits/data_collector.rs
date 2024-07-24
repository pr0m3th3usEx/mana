use futures::Stream;
use thiserror::Error;

use std::fmt::Debug;

#[derive(Debug, Error)]
pub enum DataCollectorError {
    #[error("ServiceError: {0}")]
    ServiceError(String),
}

pub type DataCollectorResult<T> = std::result::Result<T, DataCollectorError>;

pub trait DataCollector<T> {
    // Start standalone job
    async fn start(&self) -> impl Stream<Item = DataCollectorResult<T>>;

    async fn stop(&self);
}
