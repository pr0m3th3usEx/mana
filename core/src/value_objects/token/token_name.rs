#[derive(Debug, Default)]
pub struct TokenName {
    name: String,
}

impl TokenName {
    pub fn new(name: impl ToString) -> Result<Self, &'static str> {
        let name = name.to_string();

        if name.is_empty() {
            return Err("TokenName: name cannot be empty");
        }
        Ok(Self { name })
    }

    pub fn value(&self) -> String {
        self.name.clone()
    }
}
