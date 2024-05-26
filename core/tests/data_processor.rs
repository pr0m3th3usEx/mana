use core::traits::data_processor::DataProcessor;

use chrono::{DateTime, Utc};

pub struct MockDataProcessor {}

impl DataProcessor<String> for MockDataProcessor {
    fn add_data(&self, timestamp: DateTime<Utc>, data: String) {
        todo!()
    }

    fn register_metric(&self, metric: impl core::traits::metric::Metric) {
        todo!()
    }

    async fn update_metrics(&self) {
        todo!()
    }

    fn update_period(&self) {
        todo!()
    }

    fn reset_period(&self) {
        todo!()
    }

    fn reset(&self) {
        todo!()
    }

    fn metrics_history(
        &self,
    ) -> &std::collections::HashMap<
        chrono::prelude::DateTime<chrono::prelude::Utc>,
        std::collections::HashMap<String, f64>,
    > {
        todo!()
    }
}
