use crate::value_objects::token::{token_address::TokenAddress, token_name::TokenName};

#[derive(Debug, Default)]
pub struct TokenInfo {
    name: TokenName,
    decimals: u8,
    address: TokenAddress,
}
