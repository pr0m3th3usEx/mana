use std::collections::HashMap;

use chrono::{DateTime, Utc};
use thiserror::Error;

use crate::traits::history::History;

#[derive(Debug, Error)]
pub enum DataPointHistoryError {
    InvalidData,
}

pub struct DataPointHistory<T> {
    history: HashMap<DateTime<Utc>, T>
}

impl<T> History<T> for DataPointHistory<T> {
    type InsertError = DataPointHistoryError;

    fn insert(&mut self, timestamp: DateTime<Utc>, data: T) -> Result<(), Self::InsertError> {
        self.history.insert(timestamp, data);

        Ok(())
    }
}