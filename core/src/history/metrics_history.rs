use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use thiserror::Error;

use crate::traits::history::History;

#[derive(Debug, Error)]
pub enum MetricsHistoryError {
    #[error("Invalid data format")]
    InvalidData,
}

pub struct MetricsHistory {
    metrics: HashSet<String>,
    history: HashMap<DateTime<Utc>, HashMap<String, f64>>,
}

impl From<HashSet<String>> for MetricsHistory {
    fn from(metrics: HashSet<String>) -> Self {
        Self {
            metrics,
            history: HashMap::new(),
        }
    }
}

impl History<HashMap<String, f64>> for MetricsHistory {
    fn insert(
        &mut self,
        timestamp: DateTime<Utc>,
        data: HashMap<String, f64>,
    ) -> Result<(), Self::InsertError> {
        if !data.keys().all(|k| self.metrics.contains(k)) {
            return Err(MetricsHistoryError::InvalidData);
        }

        self.history.insert(timestamp, data);
        Ok(())
    }

    type InsertError = MetricsHistoryError;
}
