use crate::{entities::point::Point, traits::metric::Metric};

pub struct Ema {
    // Number of periods
    n: f64,

    // Smoothing factor
    k: f64,

    // Current value
    current: Option<f64>,
}

impl Ema {
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

impl Metric for Ema {
    async fn compute(&mut self, points: &[Point], period: usize) {
        // Should not compute anything if not enough values
        if points.len() < period || period < self.n as usize {
            return;
        }

        let Some(current) = self.current else {
            // Initial calculation
            self.current = Some(
                points[..self.n as usize]
                    .iter()
                    .fold(0f64, |acc, e| acc + e.close_price)
                    / self.n,
            );
            return;
        };

        self.current = Some((points.get(period).unwrap().close_price - current) * self.k + current);
    }

    fn value(&self) -> Option<f64> {
        self.current
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{entities::point::Point, traits::metric::Metric};

    use super::Ema;

    #[rstest]
    #[case(2, vec![2f64, 4f64, 6f64, 8f64, 12f64, 14f64, 16f64, 18f64, 20f64], 18.989026063100138)]
    #[tokio::test]
    async fn test_compute_ema(
        #[case] n: u64,
        #[case] points: Vec<f64>,
        #[case] expected_value: f64,
    ) {
        let mut ema = Ema::new(n);
        let points: Vec<Point> = points
            .into_iter()
            .map(|e| Point {
                close_price: e,
                volume_base: 0f64,
                volume_target: 0f64,
            })
            .collect();

        for _i in 0..points.len() {
            ema.compute(&points, _i).await;
        }

        assert_eq!(ema.value(), Some(expected_value));
    }
}
