#[derive(Debug)]
pub struct Point {
    // Closing price for the time period
    pub close_price: f64,

    // Volume transacted of target currency / token
    pub volume_target: f64,

    // Volume transacted of the base currency / token
    pub volume_base: f64,
    // Index liquidity ?
}
