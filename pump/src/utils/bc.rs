use mana_core::value_objects::token::token_address::TokenAddress;
use solana_sdk::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;

use crate::{PUMP_BONDING_CURVE_SEED, PUMP_PROGRAM_ADDRESS};

pub struct PumpBondingCurve(pub Pubkey, pub Pubkey);

/// Get Owner Account And Bonding Curve token account addresses
///
/// Arguments
/// - mint: [`TokenAddress`]
///
/// Returns
/// [`PumpBondingCurve`]: Pump bonding curve addresses
pub fn get_bonding_curve_info(mint: &TokenAddress) -> PumpBondingCurve {
    let (bc_owner, _) = Pubkey::find_program_address(
        &[PUMP_BONDING_CURVE_SEED.as_bytes(), &mint.value().to_bytes()],
        &PUMP_PROGRAM_ADDRESS,
    );

    let bc_token_account = get_associated_token_address(&bc_owner, &mint.value());

    PumpBondingCurve(bc_owner, bc_token_account)
}

#[cfg(test)]
mod tests {
    use mana_core::value_objects::token::token_address::TokenAddress;
    use solana_sdk::pubkey;

    use super::get_bonding_curve_info;

    #[test]
    fn tests_get_bonding_curve_info() {
        let mint = TokenAddress::new("2v7DzPnWksFL4exVB4mbvW5YV7n7PFYsJXyA9bZjpump").unwrap();
        let bc = get_bonding_curve_info(&mint);

        assert_eq!(
            bc.0,
            pubkey!("7nqaXeZtwHGQdM8uRHypwK42UwuykyJVXrbmaSGiMV2o")
        );
        assert_eq!(
            bc.1,
            pubkey!("CKFqK7MVxC67EdhBbhfNDAvCVj3S8TM3i2A1RkpsEkcf")
        );
    }
}
