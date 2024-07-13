#[derive(Debug, Clone, Copy)]
pub struct Amount {
    decimals: u8,
    value: f64,
}

impl Default for Amount {
    fn default() -> Self {
        Self {
            value: 1f64,
            decimals: 0,
        }
    }
}

impl Amount {
    pub fn new(value: f64, decimals: u8) -> Result<Self, &'static str> {
        if value <= 0f64 {
            return Err("Amount: non positive value");
        }

        Ok(Self { value, decimals })
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    pub fn no_decimals_value(&self) -> u64 {
        (self.value * 10f64.powi(self.decimals as i32)).trunc() as u64
    }
}
