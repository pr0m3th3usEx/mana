use super::configuration::bot_settings::DEFAULT_SLIPPAGE_TOLERANCE;

#[derive(Debug, Clone, Copy)]
pub struct SlippageTolerance {
    value: f64,
}

impl SlippageTolerance {
    pub fn new(value: f64) -> Result<Self, &'static str> {
        if value <= 0f64 || value > 1f64 {
            return Err("SlippageTolerance: should be between 0 and 1");
        }

        Ok(Self { value })
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

impl Default for SlippageTolerance {
    fn default() -> Self {
        Self {
            value: DEFAULT_SLIPPAGE_TOLERANCE,
        }
    }
}
