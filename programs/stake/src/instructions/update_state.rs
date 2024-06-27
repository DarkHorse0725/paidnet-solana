use anchor_lang::prelude::*;

use crate::{AppState, APP_STATE_SEED};

#[derive(Accounts)]
pub struct UpdateState<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    // app state account, signer must be owner of app state
    #[account(
      mut,
      seeds = [APP_STATE_SEED],
      bump,
      has_one = owner,
    )]
    pub app_state: Box<Account<'info, AppState>>,
    pub system_program: Program<'info, System>,
}

/**
 * owner can change reward amount per block
 */
pub fn update_state_handler(ctx: Context<UpdateState>, reward_per_block: u64) -> Result<()> {
    let app_state: &mut Box<Account<AppState>> = &mut ctx.accounts.app_state;
    app_state.reward_per_block = reward_per_block;
    Ok(())
}
