use crate::entities::point::Point;

pub trait Metric {
    /// Computes a new value for the metric
    /// Args:
    /// - point: [`Vec<Point>`]: Reference to all points stored in history
    ///
    /// Return:
    /// Nothing, performs computation
    fn compute(
        &mut self,
        points: &[Point],
        period: usize,
    ) -> impl std::future::Future<Output = ()> + Send;

    /// Current metric value
    ///
    /// Return:
    /// [`Option<f64>`] : value of the metric, if existing
    fn value(&self) -> Option<f64>;
}
