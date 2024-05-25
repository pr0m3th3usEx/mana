use crate::traits::metric::{Metric, MetricResult};

pub struct RSI {
    n: f64,
    current: Option<f64>,
}

impl RSI {
    pub fn new(n: u64) -> Self {
        Self {
            n: n as f64,
            current: None,
        }
    }
}

impl Metric for RSI {
    fn value(&self) -> Option<f64> {
        self.current
    }

    async fn compute(&mut self, points: &[f64], period: usize) -> MetricResult<()> {
        if points.len() < period + 1 || period < (self.n as usize - 1) {
            return Ok(());
        }

        // should only take the slice of the n-last points
        let slice = &points[((period + 1) - self.n as usize)..period + 1];

        // Calculate average gain & loss
        let mut avg_gain_sum = 0f64;
        let mut avg_loss_sum = 0f64;
        let mut t0 = 0f64;
        for (index, point) in slice.iter().enumerate() {
            if index == 0 {
                t0 = *point;
                continue;
            }

            if point - t0 > 0f64 {
                avg_gain_sum += point - t0;
            } else {
                avg_loss_sum += (point - t0).abs();
            }

            t0 = *point;
        }

        let avg_gain = avg_gain_sum / self.n;
        let avg_loss = avg_loss_sum / self.n;

        // Calculate relative strength
        let rs = avg_gain / avg_loss;

        self.current = Some(100f64 - (100f64 / (1f64 + rs)));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::traits::metric::Metric;

    use super::RSI;

    #[rstest]
    #[case(14, vec![45f64, 46f64, 47f64, 44f64, 43f64, 42f64, 44f64, 45f64, 46f64, 48f64, 47f64, 49f64, 50f64, 51f64], 66.66666666666666)]
    #[tokio::test]
    async fn test_compute_rsi(
        #[case] n: u64,
        #[case] points: Vec<f64>,
        #[case] expected_result: f64,
    ) {
        let mut rsi = RSI::new(n);

        for i in 0..points.len() {
            let _ = rsi.compute(&points, i).await;
        }

        assert_eq!(rsi.value(), Some(expected_result));
    }
}
