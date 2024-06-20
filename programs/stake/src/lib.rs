pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("4w9TECYbA3bpQ8eDZbggznQLgssPACDHFM3BQMWr3vot");

#[program]
pub mod stake {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        reward_per_block: u64,
        reward_decimals: u8,
        stake_decimals: u8,
        is_token2: bool,
        bump: u8,
    ) -> Result<()> {
        initialize_handler(
            ctx,
            reward_per_block,
            reward_decimals,
            stake_decimals,
            is_token2,
            bump,
        )
    }

    pub fn update_state(ctx: Context<UpdateState>, reward_per_block: u64) -> Result<()> {
        update_state_handler(ctx, reward_per_block)
    }

    pub fn fund_reward(ctx: Context<FundReward>, amount: u64) -> Result<()> {
        fund_reward_handler(ctx, amount)
    }

    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        claim_reward_handler(ctx)
    }

    pub fn reward_view(ctx: Context<RewardView>) -> Result<u64> {
        reward_view_handler(ctx)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        stake_handler(ctx, amount)
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        unstake_handler(ctx, amount)
    }
}
