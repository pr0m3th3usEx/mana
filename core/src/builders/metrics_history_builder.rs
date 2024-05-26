use std::collections::HashSet;

use crate::{history::metrics_history::MetricsHistory, traits::builder::Builder};

#[derive(Debug, Default)]
pub struct MetricsHistoryBuilder {
    metrics_list: HashSet<String>,
}

#[derive(Debug)]
pub enum MetricsHistoryBuilderError {
    MetricAlreadyAdded(String),
}

impl std::fmt::Display for MetricsHistoryBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl MetricsHistoryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    // TODO: improve metric addition to pre-sort metric computation in the DataProcessor
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
