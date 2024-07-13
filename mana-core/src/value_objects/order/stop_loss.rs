#[derive(Debug)]
pub enum StopLossOrder {
    /* Price floor where the assets should be sold */
    Fixed { floor: f64 },
    /* Fixed pourcentage to sell if current price drops that much */
    Trailing { max_drop: f64 },
    /* Aimed performance in pourcentage from the buying price  */
    Performance { profit: f64 },
    /* Period (in seconds) */
    TimeBased { duration: usize },
}

impl StopLossOrder {
    pub fn should_trigger(_current_price: f64) -> bool {
        todo!()
    }
}
