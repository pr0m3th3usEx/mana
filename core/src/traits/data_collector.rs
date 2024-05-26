use std::{error::Error, fmt::Debug};

use super::observability::publisher::Publisher;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Event {
    New,
}

#[derive(Debug)]
pub enum DataCollectorError {
    ServiceError(Box<dyn Error>),
}

impl std::fmt::Display for DataCollectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type DataCollectorResult<T> = std::result::Result<T, DataCollectorError>;

pub trait DataCollector<T>: Publisher<Event, T> {
    // Collects data collection from source
    async fn consume(&self) -> DataCollectorResult<Option<T>>;
}
