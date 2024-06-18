use anchor_lang::prelude::*;

#[account]
pub struct Buyer {
  pub principal: u64,  //purchas amount(based on purchase token)
  pub fee: u64, // based on purchase token
  pub withdrawn: u64,
  // vesting
  pub total_amount: u64,
  pub claimed_amount: u64,
}