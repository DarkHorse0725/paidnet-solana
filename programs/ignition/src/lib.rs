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
        uints: [u64; 18],
        offer_decimals: u8,
        purchase_decimals: u8,
        is_token22: bool,
        private: bool,
        bump: u8,
    ) -> Result<()> {
        create_pool_handler(
            ctx,
            uints,
            offer_decimals,
            purchase_decimals,
            is_token22,
            private,
            bump,
        )
    }

    pub fn fund_offer(ctx: Context<FundOffer>, amount: u64) -> Result<()> {
        fund_offer_handler(ctx, amount)
    }

    pub fn buy_in_early_pool(ctx: Context<BuyInEarlyPool>, amount: u64) -> Result<()> {
        buy_in_early_pool_handler(ctx, amount)
    }

    pub fn buy_in_open_pool(ctx: Context<BuyInOpenPool>, amount: u64) -> Result<()> {
        buy_in_open_pool_handler(ctx, amount)
    }

    pub fn claim_offer(ctx: Context<ClaimOffer>) -> Result<()> {
        claim_offer_handler(ctx)
    }

    pub fn toggle_claimable(ctx: Context<ToggleClaimable>) -> Result<()> {
        toggle_claimable_handler(ctx)
    }

    pub fn withdraw_offer(ctx: Context<WithdrawOffer>) -> Result<()> {
        withdraw_offer_handler(ctx)
    }

    pub fn withdraw_purchase(ctx: Context<WithdrawPurchase>) -> Result<()> {
        withdraw_purchase_handler(ctx)
    }

    pub fn refund_purchase(ctx: Context<RefundPurchase>) -> Result<()> {
        refund_purchase_handler(ctx)
    }

    pub fn emergency_cancel(ctx: Context<EmergencyCancel>) -> Result<()> {
        emergency_cancel_handler(ctx)
    }

    pub fn update_tge_date(ctx: Context<UpdateTgeDate>, tge_date: i64) -> Result<()> {
        update_tge_date_handler(ctx, tge_date)
    }

}
