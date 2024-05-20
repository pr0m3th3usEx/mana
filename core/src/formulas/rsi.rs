use crate::{entities::point::Point, traits::metric::Metric};

pub struct Rsi {
    n: f64,
    current: Option<f64>,
}

impl Rsi {
    pub fn new(n: u64) -> Self {
        Self {
            n: n as f64,
            current: None,
        }
    }
}

impl Metric for Rsi {
    fn value(&self) -> Option<f64> {
        self.current
    }

    async fn compute(&mut self, points: &[Point], period: usize) {
        if points.len() < period || period < (self.n as usize - 1) {
            return;
        }

        // should only take the slice of the n-last points
        let slice = &points[((period + 1) - self.n as usize)..period + 1];

        // Calculate average gain & loss
        let mut avg_gain_sum = 0f64;
        let mut avg_loss_sum = 0f64;
        let mut t0 = 0f64;
        for (index, point) in slice.iter().enumerate() {
            if index == 0 {
                t0 = point.close_price;
                continue;
            }

            if point.close_price - t0 > 0f64 {
                avg_gain_sum += point.close_price - t0;
            } else {
                avg_loss_sum += (point.close_price - t0).abs();
            }

            t0 = point.close_price;
        }

        let avg_gain = avg_gain_sum / self.n;
        let avg_loss = avg_loss_sum / self.n;

        // Calculate relative strength
        let rs = avg_gain / avg_loss;

        self.current = Some(100f64 - (100f64 / (1f64 + rs)))
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{entities::point::Point, traits::metric::Metric};

    use super::Rsi;

    #[rstest]
    #[case(14, vec![45f64, 46f64, 47f64, 44f64, 43f64, 42f64, 44f64, 45f64, 46f64, 48f64, 47f64, 49f64, 50f64, 51f64], 66.66666666666666)]
    #[tokio::test]
    async fn test_compute_rsi(
        #[case] n: u64,
        #[case] points: Vec<f64>,
        #[case] expected_result: f64,
    ) {
        let mut rsi = Rsi::new(n);
        let points: Vec<Point> = points
            .into_iter()
            .map(|e| Point {
                close_price: e,
                volume_base: 0f64,
                volume_target: 0f64,
            })
            .collect();

        for _i in 0..points.len() {
            rsi.compute(&points, _i).await;
        }

        assert_eq!(rsi.value(), Some(expected_result));
    }
}
