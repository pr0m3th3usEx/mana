#[derive(Debug, Copy, Clone)]
pub struct PumpTokenLiquidity {
    // Tokens available in the bounding curve
    tokens: f64,

    // SOL available in the bounding curve
    sol: f64,

    // Price per token
    price_per_token: f64,
}

impl PumpTokenLiquidity {
    pub fn new(tokens: f64, sol: f64, price_per_token: f64) -> Self {
        Self {
            tokens,
            sol,
            price_per_token,
        }
    }

    pub fn sol(&self) -> f64 {
        self.sol
    }

    pub fn tokens(&self) -> f64 {
        self.tokens
    }

    pub fn price_per_token(&self) -> f64 {
        self.price_per_token
    }
}
