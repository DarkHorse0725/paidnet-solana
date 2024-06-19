use anchor_lang::prelude::*;

#[account]
pub struct AppState {
  pub total_staked: u128,
  pub reward_per_block: u64,
  pub reward_amount: u128,
  pub staker_counts: u64,
  pub owner: Pubkey,
  pub initialized: bool,
  pub reward_mint: Pubkey,
  pub stake_mint: Pubkey,
  pub bump: u8,
}