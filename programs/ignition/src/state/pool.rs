use anchor_lang::prelude::*;

#[account]
pub struct Pool {
  pub owner: Pubkey,
  pub offer_token: OfferToken,
  pub purchase_token: PurchaseToken,
  pub total_raise_amount: u64,
  pub total_collect_amount: u64,
  pub total_sold: u64,
  pub token_fee_percentage: u16,
  // early pool
  pub max_buy_in_early_pool: u64,
  pub early_start: i64,
  pub early_end: i64,
  pub collect_in_early_pool: u64,
  pub sold_in_early_pool: u64,
  pub early_pool_proportion: u16,
  pub early_participant_fee: u16,
  // open pool
  pub max_buy_for_kyc_user: u64,
  pub max_buy_for_not_kyc_user: u64,
  pub open_start: i64,
  pub open_end: i64,
  pub open_pool_proportion: u16,
  pub open_participant_fee: u16,

  // vesting
  pub tge_date: i64,
  pub tge_percentage: u16,
  pub vesting_cliff: i64,
  pub vesting_frequency: i64,
  pub number_of_vesting: u16,
  pub total_funded_amount: u64,
  pub funded: bool,
  pub claimable: bool,
  pub emergency_cancelled: bool,
  pub private_raise: bool,

  pub is_token22: bool,
  pub bump: u8,
}

impl Pool {
    pub fn calculate_offer_amount(&self, purchase_amount: u64) -> u64 {
      let offer_amount: u64 = purchase_amount * self.offer_token.rate * (10 ^ self.offer_token.decimals as u64) / (10 ^ self.purchase_token.decimals as u64);
      return offer_amount;
    }
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct OfferToken {
  pub rate: u64,
  pub mint: Pubkey,
  pub decimals: u8,
}

#[derive(Debug, Clone, AnchorDeserialize, AnchorSerialize)]
pub struct PurchaseToken {
  pub mint: Pubkey,
  pub decimals: u8,
}