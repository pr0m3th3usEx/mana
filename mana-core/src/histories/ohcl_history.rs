use std::collections::BTreeMap;

use chrono::{DateTime, Utc};

use crate::{traits::history::History, value_objects::ohlc::Ohlc};

#[derive(Default)]
pub struct OhclHistory {
    history: BTreeMap<DateTime<Utc>, Ohlc>,
}

impl OhclHistory {
    pub fn new() -> Self {
        Self::default()
    }
}

impl History<Ohlc> for OhclHistory {
    type InsertError = String;

    async fn insert(
        &mut self,
        timestamp: DateTime<Utc>,
        data: Ohlc,
    ) -> Result<(), Self::InsertError> {
        self.history.insert(timestamp, data);

        Ok(())
    }

    async fn get(&self, timestamp: &DateTime<Utc>) -> Result<Option<Ohlc>, Self::InsertError> {
        Ok(self.history.get(timestamp).copied())
    }

    async fn get_all(
        &self,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
    ) -> Result<BTreeMap<DateTime<Utc>, Ohlc>, Self::InsertError> {
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
