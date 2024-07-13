use solana_sdk::signature::Keypair;

#[derive(Debug)]
pub struct Transactor {
    keypair: Keypair,
}

impl Transactor {
    pub fn new(keypair: impl TryInto<Keypair>) -> Result<Self, &'static str> {
        Ok(Self {
            keypair: keypair
                .try_into()
                .map_err(|_| "Transactor: invalid keypair")?,
        })
    }

    pub fn value(&self) -> &Keypair {
        &self.keypair
    }
}
