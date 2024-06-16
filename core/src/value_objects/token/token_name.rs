#[derive(Debug, Default)]
pub struct TokenName {
    name: String,
}

impl TokenName {
    pub fn new(name: String) -> Result<Self, &'static str> {
        if name.is_empty() {
            return Err("name cannot be empty");
        }
        Ok(Self { name })
    }

    pub fn value(&self) -> String {
        self.name.clone()
    }
}
