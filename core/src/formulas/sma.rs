use crate::{
    maths,
    traits::metric::{Metric, MetricResult},
};

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
        values: &[f64],
        period: usize, // 0 == First period
    ) -> MetricResult<()> {
        if values.len() < period {
            // Panics or send Err via Result<>
            return Ok(());
        }

        // should only take the slice of the n-last values
        let slice = if period < (self.n as usize) - 1 {
            values
        } else {
            &values[((period + 1) - self.n as usize)..period + 1]
        };

        // let divider = if period < (self.n as usize) - 1 {
        //     (period + 1) as f64
        // } else {
        //     self.n
        // };

        let average = maths::average(slice)?;

        self.current = Some(average);

        Ok(())
    }

    fn value(&self) -> Option<f64> {
        self.current
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::traits::metric::Metric;

    #[rstest]
    #[case(vec![1f64, 2f64, 3f64], 5, 2f64)]
    #[case(vec![1f64, 1f64, 1f64, 1f64, 1f64, 1f64], 6, 1f64)]
    #[case(vec![1f64, 2f64, 3f64, 4f64, 5f64, 6f64], 2, 5.5)]
    #[tokio::test]
    async fn test_sma(#[case] points: Vec<f64>, #[case] n: u64, #[case] expected_value: f64) {
        use crate::formulas::sma::SMA;

        let mut sma = SMA::new(n);

        assert!(sma.value().is_none());

        for i in 0..points.len() {
            let _ = sma.compute(&points, i).await;
        }

        assert_eq!(sma.value(), Some(expected_value));
    }
}
