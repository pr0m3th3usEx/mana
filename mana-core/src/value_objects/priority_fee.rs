use super::configuration::bot_settings::DEFAULT_PRIORITY_FEE;

#[derive(Debug, Clone, Copy)]
pub struct PriorityFee {
    value: f64,
}

impl PriorityFee {
    pub fn new(value: f64) -> Result<Self, &'static str> {
        if value <= 0f64 {
            return Err("PriorityFee: cannot have a negative value");
        }

        Ok(Self { value })
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

impl Default for PriorityFee {
    fn default() -> Self {
        Self {
            value: DEFAULT_PRIORITY_FEE,
        }
    }
}
