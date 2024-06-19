use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::AUTHORITY_SEED;

#[derive(Accounts)]
pub struct FundReward<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub reward_mint: Box<Account<'info, Mint>>,

    #[account(mut, token::mint = reward_mint)]
    pub token_account: Box<Account<'info, TokenAccount>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(seeds = [AUTHORITY_SEED, reward_mint.key().as_ref()], bump)]
    pub authority: AccountInfo<'info>,

    #[account(
    mut,
    token::mint = reward_mint,
    token::authority = authority,
  )]
    pub reward_port: Box<Account<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
}

impl<'info> FundReward<'info> {
    fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.token_account.to_account_info(),
                to: self.reward_port.to_account_info(),
                authority: self.signer.to_account_info(),
            },
        )
    }
}

pub fn fund_reward_handler(ctx: Context<FundReward>, amount: u64) -> Result<()> {
    token::transfer(ctx.accounts.transfer_ctx(), amount)?;
    Ok(())
}
