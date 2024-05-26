use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};

use crate::traits::history::History;

#[derive(Debug)]
pub enum MetricsHistoryError {
    InvalidData,
}

impl std::fmt::Display for MetricsHistoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for MetricsHistoryError {}

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
