use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use stake::cpi::fund_reward;
use stake::program::Stake;
use stake::cpi::accounts::FundReward;

use crate::Pool;

#[derive(Accounts)]
pub struct WithdrawPurchase<'info> {
  #[account(mut)]
  pub owner: Signer<'info>,

  pub purchase_mint: Box<Account<'info, Mint>>,

  /// CHECK: This is not dangerous because we don't read or write from this account
  #[account(seeds = [b"authority", pool.key().as_ref()], bump)]
  pub authority: AccountInfo<'info>,

  #[account(mut)]
  pub pool: Box<Account<'info, Pool>>,

  #[account(
      mut,
      token::mint = purchase_mint,
      token::authority = authority
  )]
  pub purchase_vault: Box<Account<'info, TokenAccount>>,

  #[account(mut)]
  pub reward_pot: Box<Account<'info, TokenAccount>>,
  pub token_program: Program<'info, Token>,
  pub stake_program: Program<'info, Stake>
}


impl <'info> WithdrawPurchase <'info> {
    fn fund_ctx(&self) -> CpiContext<'_, '_, '_, 'info, FundReward<'info>>  {
      CpiContext::new(
        self.stake_program.to_account_info(),
        FundReward {
            reward_mint: self.purchase_mint.to_account_info(),
            signer: self.owner.to_account_info(),
            token_account: self.purchase_vault.to_account_info(),
            reward_port: self.reward_pot.to_account_info(),
            token_program: self.token_program.to_account_info(),
        }
      )
    }
}

pub fn withdraw_purchase_handler(
  ctx: Context<WithdrawPurchase>,
) -> Result<()> {
  fund_reward(ctx.accounts.fund_ctx(), 10)?;
  Ok(())
}