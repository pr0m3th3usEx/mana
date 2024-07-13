use std::collections::BTreeMap;

use chrono::{DateTime, Utc};

pub trait DataProcessor<T> {
    // Add new data point in history
    async fn update(&mut self, timestamp: DateTime<Utc>, data: T);

    // Reset everything (DataPoints history, Metrics history, Period)
    fn reset(&mut self);

    // Get historical data stored
    fn history(&self) -> &BTreeMap<DateTime<Utc>, T>;

    // Get period
    fn period(&self) -> u64;
}
