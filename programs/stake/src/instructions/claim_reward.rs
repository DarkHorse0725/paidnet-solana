use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::{AppState, Staker, AUTHORITY_SEED};

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub reward_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub user_reward_token: Box<Account<'info, TokenAccount>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(seeds = [AUTHORITY_SEED, app_state.key().as_ref()], bump)]
    pub authority: AccountInfo<'info>,

    #[account(
      mut,
      token::mint = reward_mint,
    )]
    pub reward_port: Account<'info, TokenAccount>,

    #[account(mut, has_one = user)]
    pub staker: Box<Account<'info, Staker>>,
    pub token_program: Program<'info, Token>,
    pub app_state: Box<Account<'info, AppState>>,
}

impl<'info> ClaimReward<'info> {
    fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.reward_port.to_account_info(),
                to: self.user_reward_token.to_account_info(),
                authority: self.authority.to_account_info(),
            },
        )
    }
}

pub fn claim_reward_handler(ctx: Context<ClaimReward>) -> Result<()> {
    let amount: u64 = ctx.accounts.app_state.calculate_reward(
        ctx.accounts.staker.total_amount,
        ctx.accounts.staker.last_update,
    );
    let seeds: &[&[u8]; 3] = &[
        AUTHORITY_SEED,
        ctx.accounts.app_state.to_account_info().key.as_ref(),
        &[ctx.accounts.app_state.bump],
    ];
    let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];
    transfer(ctx.accounts.transfer_ctx().with_signer(signer_seeds), amount)?;
    let staker: &mut Box<Account<Staker>> = &mut ctx.accounts.staker;
    let now: i64 = Clock::get().unwrap().unix_timestamp;
    staker.last_update = now;
    Ok(())
}
