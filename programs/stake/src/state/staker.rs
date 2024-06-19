use anchor_lang::prelude::*;

#[account]
pub struct Staker {
  pub total_amount: u64,
  pub user: Pubkey,
}