use core::traits::data_processor::DataProcessor;
use std::collections::BTreeMap;

use chrono::{DateTime, Utc};

#[derive(Default)]
pub struct MockDataProcessor<T: Default> {
    history: BTreeMap<DateTime<Utc>, T>,
    period: u64,
}

impl<T: Default> MockDataProcessor<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl DataProcessor<String> for MockDataProcessor<String> {
    async fn update(&mut self, timestamp: DateTime<Utc>, data: String) {
        self.history.insert(timestamp, data);
        self.period += 1;
    }

    fn reset(&mut self) {
        self.period = 0;
    }

    fn history(&self) -> &std::collections::BTreeMap<DateTime<Utc>, String> {
        &self.history
    }

    fn period(&self) -> u64 {
        self.period
    }
}
