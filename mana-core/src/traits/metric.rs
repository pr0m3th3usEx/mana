use std::error::Error;

use crate::utils::maths::MathError;

#[derive(Debug)]
pub enum MetricError {
    ComputationError(Box<dyn Error>),
    MissingDependency,
    LockError,
}

pub type MetricResult<T> = Result<T, MetricError>;

pub trait Metric {
    /// Computes a new value for the metric
    /// Args:
    /// - point: [`Vec<Point>`]: Reference to all points stored in history
    ///
    /// Return:
    /// Nothing, performs computation
    fn compute(
        &mut self,
        points: &[f64],
        period: usize,
    ) -> impl std::future::Future<Output = MetricResult<()>> + Send;

    /// Current metric value
    ///
    /// Return:
    /// [`Option<f64>`] : value of the metric, if existing
    fn value(&self) -> Option<f64>;
}

impl std::fmt::Display for MetricError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetricError::ComputationError(err) => write!(f, "MetricError: {:?}", err),
            other => write!(f, "MetricError: {:?}", other),
        }
    }
}

impl From<MathError> for MetricError {
    fn from(value: MathError) -> Self {
        MetricError::ComputationError(Box::new(value))
    }
}
