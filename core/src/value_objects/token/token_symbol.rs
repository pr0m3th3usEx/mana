#[derive(Debug, Default)]
pub struct TokenSymbol {
    symbol: String,
}

impl TokenSymbol {
    pub fn new(symbol: impl ToString) -> Result<Self, &'static str> {
        let symbol = symbol.to_string();

        if symbol.is_empty() {
            return Err("TokenSymbol: token symbol can't be empty");
        }
        Ok(Self { symbol })
    }

    pub fn value(&self) -> String {
        self.symbol.clone()
    }
}
