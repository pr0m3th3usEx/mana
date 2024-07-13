#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    VARIANCE,
    AVERAGE,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MathError {
    ImpossibleComputation(Operation),
}

impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MathError::ImpossibleComputation(operation) => {
                write!(f, "Impossible Computation: {}", operation)
            }
        }
    }
}

impl std::error::Error for MathError {}

pub fn variance(e: f64, apexs: &[f64]) -> Result<f64, MathError> {
    let n = apexs.len();
    if n == 0 {
        return Err(MathError::ImpossibleComputation(Operation::VARIANCE));
    }

    let result = apexs.iter().fold(0f64, |acc, v| acc + (v - e).powf(2f64)) / (n as f64);

    Ok(result)
}

pub fn average(apexs: &[f64]) -> Result<f64, MathError> {
    let n = apexs.len();
    if n == 0 {
        return Err(MathError::ImpossibleComputation(Operation::AVERAGE));
    }

    Ok(apexs.iter().sum::<f64>() / (n as f64))
}

pub fn standard_deviation(apexs: &[f64]) -> Result<f64, MathError> {
    let e = average(apexs)?;
    let variance = variance(e, apexs)?;

    Ok(variance.sqrt())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(vec![1.0, 1.0, 1.0, 1.0], Ok(1.0))]
    #[case(vec![1.0, 2.0, 3.0, 4.0, 5.0], Ok(3.0))]
    #[case(vec![10.0, 12.0, 23.0, 23.0, 16.0, 23.0, 21.0, 16.0], Ok(18.0))]
    #[case(vec![], Err(MathError::ImpossibleComputation(Operation::AVERAGE)))]
    fn test_average(#[case] values: Vec<f64>, #[case] expected_value: Result<f64, MathError>) {
        assert_eq!(average(&values), expected_value);
    }

    #[rstest]
    #[case(vec![1.0, 1.0, 1.0, 1.0], 1.0, Ok(0.0))]
    #[case(vec![1.0, 2.0, 3.0, 4.0, 5.0], 3.0, Ok(2.0))]
    #[case(vec![10.0, 12.0, 23.0, 23.0, 16.0, 23.0, 21.0, 16.0], 18.0, Ok(24.0))]
    #[case(vec![], 0.0, Err(MathError::ImpossibleComputation(Operation::VARIANCE)))]
    fn test_variance(
        #[case] values: Vec<f64>,
        #[case] e: f64,
        #[case] expected_value: Result<f64, MathError>,
    ) {
        assert_eq!(variance(e, &values), expected_value);
    }

    #[rstest]
    #[case(vec![1.0, 1.0, 1.0, 1.0], Ok(0.0))]
    #[case(vec![1.0, 2.0, 3.0, 4.0, 5.0], Ok(1.4142135623730951))]
    #[case(vec![10.0, 12.0, 23.0, 23.0, 16.0, 23.0, 21.0, 16.0], Ok(4.898979485566356))]
    fn test_standard_deviation(
        #[case] values: Vec<f64>,
        #[case] expected_value: Result<f64, MathError>,
    ) {
        assert_eq!(standard_deviation(&values), expected_value);
    }
}
