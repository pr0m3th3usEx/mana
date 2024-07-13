use std::error::Error;

use super::data_collector::DataCollectorError;

#[derive(Debug)]
pub enum DataRepositoryError {
    ServiceError(String),
}

impl std::fmt::Display for DataRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for DataRepositoryError {}

pub type DataRepositoryResult<T> = std::result::Result<T, DataRepositoryError>;

pub trait DataRepository<T: Clone> {
    async fn get_data(&self) -> DataRepositoryResult<Option<T>>;
}

impl From<DataRepositoryError> for DataCollectorError {
    fn from(err: DataRepositoryError) -> Self {
        Self::ServiceError(Box::new(err))
    }
}
