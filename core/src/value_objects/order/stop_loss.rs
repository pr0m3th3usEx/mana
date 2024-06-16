#[derive(Debug)]
pub enum StopLossOrder {
    /* Price floor where the assets should be sold */
    Fixed(f64),
    /* Fixed pourcentage to sell if current price drops that much */
    Trailing(f64),
    /* Price ceiling where the assets should be sold */
    Market(f64),
    /* Aimed performance in pourcentage from the buying price  */
    Performance(f64),
    /* Period (in seconds) */
    TimeBased(u64),
}

impl StopLossOrder {
    fn should_trigger(current_price: f64) -> bool {
        todo!()
    }
}
