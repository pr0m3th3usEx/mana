use crate::{entities::point::Point, traits::metric::Metric};

pub struct SMA {
    // Number of periods
    n: f64,

    // Current value
    current: Option<f64>,
}

impl SMA {
    pub fn new(n: u64) -> Self {
        Self {
            n: n as f64,
            current: None,
        }
    }
}

impl Metric for SMA {
    async fn compute(
        &mut self,
        points: &[Point],
        period: usize, // 0 == First period
    ) {
        if points.len() < period {
            // Panics or send Err via Result<>
            return;
        }

        // should only take the slice of the n-last points
        let slice = if period < (self.n as usize) - 1 {
            points
        } else {
            &points[((period + 1) - self.n as usize)..period + 1]
        };

        let divider = if period < (self.n as usize) - 1 {
            (period + 1) as f64
        } else {
            self.n
        };

        let sum = slice.iter().fold(0f64, |acc, p| acc + p.close_price);
        let average = sum / divider;

        self.current = Some(average);
    }

    fn value(&self) -> Option<f64> {
        self.current
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{entities::point::Point, traits::metric::Metric};

    #[rstest]
    #[case(vec![1f64, 2f64, 3f64], 5, 2f64)]
    #[case(vec![1f64, 1f64, 1f64, 1f64, 1f64, 1f64], 6, 1f64)]
    #[case(vec![1f64, 2f64, 3f64, 4f64, 5f64, 6f64], 2, 5.5)]
    #[tokio::test]
    async fn test_sma(#[case] points: Vec<f64>, #[case] n: u64, #[case] expected_value: f64) {
        use crate::formulas::sma::SMA;

        let mut sma = SMA::new(n);

        assert!(sma.value().is_none());

        let points: Vec<Point> = points
            .into_iter()
            .map(|e| Point {
                close_price: e,
                volume_base: 0f64,
                volume_target: 0f64,
            })
            .collect();

        for _i in 0..points.len() {
            sma.compute(&points, _i).await;
        }

        assert_eq!(sma.value(), Some(expected_value));
    }
}
