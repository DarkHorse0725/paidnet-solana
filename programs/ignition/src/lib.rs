pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("BnwF9UvkZGPzoAm8GbuUxkfgHLB9tpK5zx25tv715J6a");

#[program]
pub mod ignition {
    use super::*;

    pub fn create_pool(
        ctx: Context<CreatePool>,
        uints: [u64; 17],
        offer_decimals: u8,
        purchase_decimals: u8,
        is_token22: bool,
        bump: u8,
    ) -> Result<()> {
        create_pool_handler(ctx, uints, offer_decimals, purchase_decimals, is_token22, bump)
    }
}
