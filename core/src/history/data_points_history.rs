use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::traits::history::History;

#[derive(Debug)]
pub enum DataPointHistoryError {
    InvalidData,
}

impl std::fmt::Display for DataPointHistoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DataPointHistoryError {}

pub struct DataPointHistory<T> {
    history: HashMap<DateTime<Utc>, T>,
}

impl<T> History<T> for DataPointHistory<T> {
    type InsertError = DataPointHistoryError;

    fn insert(&mut self, timestamp: DateTime<Utc>, data: T) -> Result<(), Self::InsertError> {
        self.history.insert(timestamp, data);

        Ok(())
    }
}
