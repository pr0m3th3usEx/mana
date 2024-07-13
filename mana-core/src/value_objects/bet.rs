#[derive(Debug, Default, Clone, Copy)]
pub struct Bet {
    value: f64,
}

impl Bet {
    pub fn new(value: f64) -> Result<Self, &'static str> {
        if value <= 0f64 {
            return Err("Bet: cannot have a negative value");
        }

        Ok(Self { value })
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}
