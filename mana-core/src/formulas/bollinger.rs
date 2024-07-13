use std::sync::{Arc, RwLock};

use crate::{
    maths,
    traits::metric::{Metric, MetricError, MetricResult},
};

use super::sma::SMA;

#[derive(Debug)]
pub enum BollingerType {
    UpperBand(f64), // k factor
    LowerBand(f64), // k factor
}

impl Default for BollingerType {
    fn default() -> Self {
        Self::LowerBand(0f64)
    }
}

#[derive(Debug, Default)]
pub struct Bollinger {
    sma: Arc<RwLock<SMA>>,
    band_type: BollingerType,
    current: Option<f64>,
    n: f64,
}

impl Bollinger {
    pub fn new(band_type: BollingerType, sma: Arc<RwLock<SMA>>) -> Self {
        let n = sma.read().expect("SMA lock poisoned").period();

        Self {
            n,
            sma,
            band_type,
            current: None,
        }
    }
}

impl Metric for Bollinger {
    async fn compute(
        &mut self,
        values: &[f64], // Not used
        period: usize,  // Not used
    ) -> MetricResult<()> {
        if values.len() < period + 1 {
            // MetricError or panics;
            return Ok(());
        }

        let slice = if period < (self.n as usize) - 1 {
            values
        } else {
            &values[((period + 1) - self.n as usize)..period + 1]
        };

        let average = self
            .sma
            .read()
            .map_err(|_| MetricError::LockError)?
            .value()
            .ok_or(MetricError::MissingDependency)?;

        let variance = maths::variance(average, slice)?;
        let sd = variance.sqrt();

        println!("sma: {}, variance: {}, sd: {}", average, variance, sd);

        self.current = Some(match self.band_type {
            BollingerType::UpperBand(k) => average + k * sd,
            BollingerType::LowerBand(k) => average - k * sd,
        });

        Ok(())
    }

    fn value(&self) -> Option<f64> {
        self.current
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, RwLock};

    use rstest::rstest;

    use crate::formulas::sma::SMA;

    use super::*;

    #[rstest]
    #[case(
        BollingerType::UpperBand(2f64),
        5,
        vec![10f64,12f64, 14f64, 13f64, 15f64],
        16.24093010681705
    )]
    #[case(
        BollingerType::LowerBand(2f64),
        5,
        vec![10f64,12f64, 14f64, 13f64, 15f64],
        9.35906989318295
    )]
    #[tokio::test]
    async fn test_compute_bollinger(
        #[case] band_type: BollingerType,
        #[case] n: u64,
        #[case] points: Vec<f64>,
        #[case] expected_value: f64,
    ) {
        let sma = Arc::new(RwLock::new(SMA::new(n)));
        let mut band = Bollinger::new(band_type, Arc::clone(&sma));

        println!("n: {}", band.n);

        for i in 0..points.len() {
            let _ = sma
                .write()
                .expect("lock poisoned")
                .compute(&points, i)
                .await;
            let _ = band.compute(&points, i).await;
        }

        println!("sma: {:?}", sma.read().expect("lock poisoned").value());
        assert_eq!(band.value(), Some(expected_value));
    }
}
