use std::collections::BTreeMap;

use chrono::{DateTime, Utc};

use crate::traits::history::History;

#[derive(Default)]
pub struct TickHistory {
    history: BTreeMap<DateTime<Utc>, f64>,
}

impl TickHistory {
    pub fn new() -> Self {
        Self::default()
    }
}

impl History<f64> for TickHistory {
    type InsertError = String;

    async fn insert(
        &mut self,
        timestamp: DateTime<Utc>,
        data: f64,
    ) -> Result<(), Self::InsertError> {
        self.history.insert(timestamp, data);

        Ok(())
    }

    async fn get(&self, timestamp: &DateTime<Utc>) -> Result<Option<f64>, Self::InsertError> {
        let value = self.history.get(timestamp).copied();

        Ok(value)
    }

    async fn get_all(
        &self,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
    ) -> Result<BTreeMap<DateTime<Utc>, f64>, Self::InsertError> {
        let mut result = BTreeMap::new();

        for (&key, &value) in &self.history {
            if let Some(start) = start {
                if key <= start {
                    continue;
                }
            }
            if let Some(end) = end {
                if key >= end {
                    continue;
                }
            }
            result.insert(key, value);
        }
        Ok(result)
    }
}
