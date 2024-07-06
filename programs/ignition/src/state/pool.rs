use anchor_lang::prelude::*;

use crate::DENOMINATOR;

#[account]
pub struct Pool {
    pub owner: Pubkey,
    pub offer_token: OfferToken,
    pub purchase_token: Pubkey,
    pub total_raise_amount: u64,
    pub total_collect_amount: u64,
    pub total_sold: u64,
    pub token_fee_percentage: u16,
    pub total_fee: u64,
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
    pub udpate_tge_attempts: u16,

    pub bump: u8,
}

impl Pool {
    pub fn calculate_offer_amount(&self, purchase_amount: u64, purchase_decimals: u8, offer_decimals: u8) -> u64 {
        let offer_amount: u64 =
            purchase_amount * self.offer_token.rate / 10_u64.pow(purchase_decimals as u32) * 10_u64.pow(offer_decimals as u32);
        return offer_amount;
    }
    pub fn calculate_claimable_amount(&self, total_amount: u64, claimed_amount: u64) -> u64 {
        if claimed_amount >= total_amount {
            return 0;
        }
        let tge_amount = (total_amount * self.tge_percentage as u64) / DENOMINATOR as u64;
        let now: i64 = Clock::get().unwrap().unix_timestamp;
        if now < self.tge_date + self.vesting_cliff {
            return tge_amount - claimed_amount;
        }

        let release_index: i64 =
            (now - self.tge_date - self.vesting_cliff) / self.vesting_frequency + 1;
        if (release_index >= self.number_of_vesting as i64) || (self.vesting_frequency == 0) {
            return total_amount - claimed_amount;
        }
        let total_claimable_except_tge_amount = total_amount - tge_amount;
        return (release_index as u64 * total_claimable_except_tge_amount)
            / self.number_of_vesting as u64
            + tge_amount
            - claimed_amount;
    }
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct OfferToken {
    pub rate: u64,
    pub mint: Pubkey,
}

