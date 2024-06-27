use anchor_lang::prelude::*;

use crate::{AppState, Staker};

#[derive(Accounts)]
pub struct RewardView<'info> {
    pub staker: Box<Account<'info, Staker>>,
    pub app_state: Box<Account<'info, AppState>>,
}

/**
 * this is view function like evm
 * this will show user's reward amount
 */
pub fn reward_view_handler(ctx: Context<RewardView>) -> Result<u64> {
    let amount: u64 = ctx.accounts.app_state.calculate_reward(
        ctx.accounts.staker.total_amount,
        ctx.accounts.staker.last_update,
    );
    Ok(amount)
}
