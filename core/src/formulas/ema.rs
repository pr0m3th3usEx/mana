use crate::traits::metric::{Metric, MetricResult};

pub struct EMA {
    // Number of periods
    n: f64,

    // Smoothing factor
    k: f64,

    // Current value
    current: Option<f64>,
}

impl EMA {
    pub fn new(n: u64) -> Self {
        Self {
            n: n as f64,
            k: Self::smooth_factor(n),
            current: None,
        }
    }

    fn smooth_factor(n: u64) -> f64 {
        2f64 / ((n as f64) + 1f64)
    }
}

impl Metric for EMA {
    async fn compute(&mut self, values: &[f64], period: usize) -> MetricResult<()> {
        // Should not compute anything if not enough values
        if values.len() < period + 1 || period < self.n as usize {
            return Ok(());
        }

        let Some(current) = self.current else {
            // Initial calculation
            self.current = Some(
                values[..self.n as usize]
                    .iter()
                    .fold(0f64, |acc, e| acc + e)
                    / self.n,
            );
            return Ok(());
        };

        self.current = Some((values.get(period).unwrap() - current) * self.k + current);

        Ok(())
    }

    fn value(&self) -> Option<f64> {
        self.current.clone()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::traits::metric::Metric;

    use super::EMA;

    #[rstest]
    #[case(2, vec![2f64, 4f64, 6f64, 8f64, 12f64, 14f64, 16f64, 18f64, 20f64], 18.989026063100138)]
    #[tokio::test]
    async fn test_compute_ema(
        #[case] n: u64,
        #[case] points: Vec<f64>,
        #[case] expected_value: f64,
    ) {
        let mut ema = EMA::new(n);
        for i in 0..points.len() {
            let _ = ema.compute(&points, i).await;
        }

        assert_eq!(ema.value(), Some(expected_value));
    }
}
