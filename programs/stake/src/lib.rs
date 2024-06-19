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

    pub fn initialize(ctx: Context<Initialize>, reward_per_block: u64, bump: u8) -> Result<()>  {
        initialize_handler(ctx, reward_per_block, bump)
    }

    pub fn update_state(ctx: Context<UpdateState>, reward_per_block: u64) -> Result<()> {
        update_state_handler(ctx, reward_per_block)
    }

    pub fn fund_reward(ctx: Context<FundReward>, amount: u64) -> Result<()> {
        fund_reward_handler(ctx, amount)
    }
}
