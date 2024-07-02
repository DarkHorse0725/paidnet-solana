use anchor_lang::prelude::*;

use crate::REWARD_DENOMINATOR;

#[account]
pub struct AppState {
  pub total_staked: u128,
  pub reward_per_block: u64,
  pub reward_amount: u128,
  pub staker_counts: u64,
  pub owner: Pubkey,
  pub initialized: bool,
  pub s_paid: Pubkey,
  pub paid: Pubkey,
  pub fuel_percentage: u16,
  pub is_token2022: bool,
  pub bump: u8,
}

impl AppState {
    pub fn calculate_reward(&self, total_amount: u64, last_update: i64) -> u64 {
      let now: i64 = Clock::get().unwrap().unix_timestamp;
      let period: i64 = last_update - now;
      let amount: u64 = total_amount * period as u64 * self.reward_per_block / REWARD_DENOMINATOR;
      return amount;
    }
}

// #[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
// pub struct RewardToken {
//     pub mint: Pubkey,
// }

// #[derive(Debug, Clone, AnchorDeserialize, AnchorSerialize)]
// pub struct StakeToken {
//     pub mint: Pubkey,
//     pub is_token2: bool,
// }
