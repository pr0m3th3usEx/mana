use crate::value_objects::token::{
    token_address::TokenAddress, token_name::TokenName, token_symbol::TokenSymbol,
};

#[derive(Debug, Default)]
pub struct TokenInfo {
    name: TokenName,
    decimals: u8,
    symbol: TokenSymbol,
    address: TokenAddress,
}

impl TokenInfo {
    pub fn new(name: TokenName, address: TokenAddress, symbol: TokenSymbol, decimals: u8) -> Self {
        Self {
            name,
            decimals,
            symbol,
            address,
        }
    }
}
