use std::collections::HashMap;

use chrono::{DateTime, Utc};

use super::metric::Metric;

pub trait DataProcessor<T> {
    // Add new data point in history
    fn add_data(&self, timestamp: DateTime<Utc>, data: T);

    // Store metric that will be further computed
    fn register_metric(&self, metric: impl Metric);

    // Update metrics
    async fn update_metrics(&self);

    // Update period
    fn update_period(&self);

    // Reset period
    fn reset_period(&self);

    // Reset everything (DataPoints history, Metrics history, Period)
    fn reset(&self);

    // Get metric history
    fn metrics_history(&self) -> &HashMap<DateTime<Utc>, HashMap<String, f64>>;
}
