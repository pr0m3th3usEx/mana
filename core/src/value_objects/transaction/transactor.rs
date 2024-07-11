use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Default)]
pub struct Transactor {
    address: Pubkey,
}

impl Transactor {
    pub fn new(address: impl TryInto<Pubkey>) -> Result<Self, &'static str> {
        Ok(Self {
            address: address
                .try_into()
                .map_err(|_| "Transactor: invalid public key")?,
        })
    }

    pub fn value(&self) -> Pubkey {
        self.address
    }
}
