use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::token::Token;
use anchor_spl::token_interface;

use crate::error::ErrorCode;
use crate::{AppState, APP_STATE_SEED, AUTHORITY_SEED};
use std::mem::size_of;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    /// CHECK:
    pub paid: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(seeds = [AUTHORITY_SEED, app_state.key().as_ref()], bump)]
    pub authority: AccountInfo<'info>,

    #[account(
      mut,
      constraint = s_paid.mint_authority == COption::Some(authority.key()),
      constraint = s_paid.freeze_authority.is_none(),
    )]
    pub s_paid: Box<InterfaceAccount<'info, token_interface::Mint>>,

    // app state of account of stake program
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


/**
 * stake program must be initialized by owner after deploying
 * @params
 *  reward amount per block
 *  reward token decimals
 *  stake token decimals
 *  is token2, true if stake token is token 2022
 *  bump of authority
 */

pub fn initialize_handler(
    ctx: Context<Initialize>,
    reward_per_block: u64,
    fuel_percent: u16,
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
    app_state.paid = ctx.accounts.paid.key();
    app_state.s_paid= ctx.accounts.s_paid.key();
    app_state.fuel_percentage = fuel_percent;
    app_state.is_token2022 = is_token2;
    app_state.bump = bump;
    Ok(())
}
