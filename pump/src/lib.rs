use solana_program::{pubkey, pubkey::Pubkey};

pub mod adapters;
pub mod pump_client;
pub mod utils;
pub mod value_objects;

pub const PUMP_PROGRAM_ADDRESS: Pubkey = pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");
pub const PUMP_BONDING_CURVE_SEED: &str = "bonding-curve";
