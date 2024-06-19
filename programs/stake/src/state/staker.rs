use anchor_lang::prelude::*;

#[account]
pub struct Staker {
  pub total_amount: u64,
  pub user: Pubkey,
  pub last_update: i64,
}