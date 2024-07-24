#[derive(Debug, Copy, Clone)]
pub struct PumpTokenLiquidity {
    // Tokens available in the bounding curve
    tokens: f64,

    // SOL available in the bounding curve
    sol: f64,
}

impl PumpTokenLiquidity {
    pub fn new(tokens: f64, sol: f64) -> Self {
        Self { tokens, sol }
    }

    pub fn sol(&self) -> f64 {
        self.sol
    }

    pub fn tokens(&self) -> f64 {
        self.tokens
    }
}
