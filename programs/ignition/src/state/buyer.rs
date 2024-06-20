use anchor_lang::prelude::*;

#[account]
pub struct Buyer {
  pub principal: u64,  //purchas amount(based on purchase token)
  pub fee: u64, // based on purchase token
  pub withdrawn: u64,
  pub purchase_in_early_pool: u64, // based on purchase token
  // vesting
  pub total_amount: u64,
  pub claimed_amount: u64,
  pub refunded: bool,
}