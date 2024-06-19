use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{AppState, APP_STATE_SEED, AUTHORITY_SEED};

#[derive(Accounts)]
pub struct UpdateState<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK:
    pub reward_mint: Box<Account<'info, Mint>>,
    /// CHECK:
    pub stake_mint: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(seeds = [AUTHORITY_SEED, app_state.key().as_ref()], bump)]
    pub authority: AccountInfo<'info>,

    #[account(
      init_if_needed,
      payer = owner,
      token::mint = reward_mint,
      token::authority = authority,
    )]
    pub reward_port: Box<Account<'info, TokenAccount>>,

    #[account(
      mut,
      seeds = [APP_STATE_SEED],
      bump,
      has_one = owner,
    )]
    pub app_state: Box<Account<'info, AppState>>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn update_state_handler(ctx: Context<UpdateState>, reward_per_block: u64) -> Result<()> {
    let app_state: &mut Box<Account<AppState>> = &mut ctx.accounts.app_state;
    app_state.reward_per_block = reward_per_block;
    app_state.reward_mint = ctx.accounts.reward_mint.key();
    app_state.stake_mint = ctx.accounts.stake_mint.key();
    Ok(())
}
