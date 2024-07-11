use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Default)]
pub struct TokenAddress {
    address: Pubkey,
}

impl TokenAddress {
    pub fn new(address: impl TryInto<Pubkey>) -> Result<Self, &'static str> {
        Ok(Self {
            address: address
                .try_into()
                .map_err(|_| "TokenAddress: Invalid public key")?,
        })
    }

    pub fn value(&self) -> Pubkey {
        self.address
    }
}
