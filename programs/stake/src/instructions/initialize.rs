use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::error::ErrorCode;
use crate::{AppState, APP_STATE_SEED, AUTHORITY_SEED, REWARD_POT_SEED};
use std::mem::size_of;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    pub reward_mint: Box<Account<'info, Mint>>,
    /// CHECK:
    pub stake_mint: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(seeds = [AUTHORITY_SEED, app_state.key().as_ref()], bump)]
    pub authority: AccountInfo<'info>,

    #[account(
      init,
      payer = creator,
      token::mint = reward_mint,
      token::authority = authority,
      seeds = [REWARD_POT_SEED],
      bump,
    )]
    pub reward_pot: Box<Account<'info, TokenAccount>>,

    #[account(
      init,
      payer = creator,
      space = size_of::<AppState>() + 8,
      seeds = [APP_STATE_SEED],
      bump,
    )]
    pub app_state: Box<Account<'info, AppState>>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_handler(
    ctx: Context<Initialize>,
    reward_per_block: u64,
    reward_decimals: u8,
    stake_decimals: u8,
    is_token2: bool,
    bump: u8,
) -> Result<()> {
    if ctx.accounts.app_state.initialized {
        return err!(ErrorCode::Initialized);
    }
    let app_state: &mut Box<Account<AppState>> = &mut ctx.accounts.app_state;
    app_state.initialized = true;
    app_state.owner = ctx.accounts.creator.key();
    app_state.reward_per_block = reward_per_block;
    app_state.staker_counts = 0;
    app_state.reward_amount = 0;
    app_state.total_staked = 0;
    app_state.reward_token.mint = ctx.accounts.reward_mint.key();
    app_state.reward_token.decimals = reward_decimals;
    app_state.stake_token.mint = ctx.accounts.stake_mint.key();
    app_state.stake_token.decimals = stake_decimals;
    app_state.stake_token.is_token2 = is_token2;
    app_state.bump = bump;
    Ok(())
}
