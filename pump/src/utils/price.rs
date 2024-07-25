const K: f64 = 30.0;
const C: f64 = 32190005730.0;

/// Get the price of a token according to the number SOL in the liquidity pool
pub fn pump_token_price(sol_pool: f64) -> f64 {
    ((K + sol_pool).powf(2.0)) / C
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    fn precision_f64(x: f64, decimals: u32) -> f64 {
        if x == 0. || decimals == 0 {
            0.
        } else {
            let shift = decimals as i32 - x.abs().log10().ceil() as i32;
            let shift_factor = 10_f64.powi(shift);

            (x * shift_factor).round() / shift_factor
        }
    }

    #[rstest]
    #[case(0.57, 0.0000000290)]
    #[case(1.254, 0.0000000303)]
    fn test_pump_token_price(#[case] sol_pool: f64, #[case] expected_price: f64) {
        assert_eq!(precision_f64(pump_token_price(sol_pool), 3), expected_price);
    }
}
