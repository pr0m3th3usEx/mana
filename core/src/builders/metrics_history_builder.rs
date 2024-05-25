// MetricsHistoryBuilder::new()
//     .add_metric("")?
//     .add_metric("")?
//     .add_metric("")?
//     .build();

// From<Vec<String>> for MetricsHiistory

use std::collections::HashSet;

use thiserror::Error;

use crate::{history::metrics_history::MetricsHistory, traits::builder::Builder};

#[derive(Debug, Default)]
pub struct MetricsHistoryBuilder {
    metrics_list: HashSet<String>,
}

#[derive(Debug, Error)]
pub enum MetricsHistoryBuilderError {
    #[error("Metric already added: {0}")]
    MetricAlreadyAdded(String),
}

impl MetricsHistoryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_metric(mut self, name: &str) -> Result<Self, MetricsHistoryBuilderError> {
        let metric_name = name.to_lowercase().to_string();
        if !self.metrics_list.insert(metric_name.clone()) {
            return Err(MetricsHistoryBuilderError::MetricAlreadyAdded(metric_name));
        }

        Ok(self)
    }
}

impl Builder<MetricsHistory> for MetricsHistoryBuilder {
    type Error = MetricsHistoryBuilderError;

    fn build(self) -> Result<MetricsHistory, Self::Error> {
        let history = MetricsHistory::from(self.metrics_list);

        Ok(history)
    }
}
