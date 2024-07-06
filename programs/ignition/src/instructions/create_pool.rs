use anchor_lang::prelude::*;
use anchor_spl::token_interface;

use std::mem::size_of;

use crate::{error::ErrCode, Pool, DENOMINATOR};

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    pub offer_mint: Box<InterfaceAccount<'info, token_interface::Mint>>,
    /// CHECK:
    pub purchase_mint: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(seeds = [b"authority", pool.key().as_ref()], bump)]
    pub authority: AccountInfo<'info>,

    #[account(
        init,
        payer = creator,
        seeds = [b"offer-vault", pool.key().as_ref()],
        bump,
        rent_exempt = enforce,
        token::mint = offer_mint,
        token::authority = authority
    )]
    pub offer_vault: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    #[account(
      init,
      payer = creator,
      space = size_of::<Pool>() + 8,
    )]
    pub pool: Box<Account<'info, Pool>>,

    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub system_program: Program<'info, System>,
}

/**
 * pool containers 2 sub pools: early pool and open pool,
 * @param uints Array of pool informaton includes:
 *  max buy for kyc user
 *  max buy for not kyc user
 *  max buy in ealry pool
 *  token fee percentage
 *  early pool participant fee
 *  open pool participant fee
 *  early pool proportion
 *  open pool proportion
 *  total raise amount
 *  early pool start time
 *  open pool start time
 *  open pool end time
 *  rate of offer token
 *  tge date
 *  tge percentage
 *  vesting cliff
 *  vesting frequency
 *  number of vesting
 * @param decimals of offer token
 * @param decimals of purchase token
 * @param is token22, true if ido token is token2022,
 * @param private, true if it is private sale
 * @Param bump, authority bump
 */

pub fn create_pool_handler(
    ctx: Context<CreatePool>,
    uints: [u64; 18],
    private: bool,
    bump: u8,
) -> Result<()> {
    let pool: &mut Box<Account<Pool>> = &mut ctx.accounts.pool;
    if uints[3] as u16 > DENOMINATOR {
        return err!(ErrCode::InvalidTokenFeePercentage);
    }
    // early pool proportion
    if uints[6] as u16 > DENOMINATOR {
        return err!(ErrCode::InvalidTokenFeePercentage);
    }
    // open pool proportion
    if uints[7] as u16 > DENOMINATOR {
        return err!(ErrCode::InvalidTokenFeePercentage);
    }
    if uints[9] > uints[10] {
        return err!(ErrCode::InvalidTime);
    }
    if uints[10] > uints[11] {
        return err!(ErrCode::InvalidTime);
    }
    if uints[11] > uints[13] {
        return err!(ErrCode::InvalidTime);
    }
    pool.max_buy_for_kyc_user = uints[0];
    pool.max_buy_for_not_kyc_user = uints[1];
    pool.max_buy_in_early_pool = uints[2];
    pool.token_fee_percentage = uints[3] as u16;
    pool.early_participant_fee = uints[4] as u16;
    pool.open_participant_fee = uints[5] as u16;
    pool.early_pool_proportion = uints[6] as u16;
    pool.open_pool_proportion = uints[7] as u16;
    pool.total_raise_amount = uints[8];
    pool.early_start = uints[9] as i64;
    pool.early_end = uints[10] as i64;
    pool.open_start = uints[10] as i64;
    pool.open_end = uints[11] as i64;

    pool.offer_token.rate = uints[12];
    pool.tge_date = uints[13] as i64;
    pool.tge_percentage = uints[14] as u16;
    pool.vesting_cliff = uints[15] as i64;
    pool.vesting_frequency = uints[16] as i64;
    pool.number_of_vesting = uints[17] as u16;
    pool.owner = ctx.accounts.creator.key();
    pool.total_collect_amount = 0;
    pool.total_sold = 0;
    pool.collect_in_early_pool = 0;
    pool.sold_in_early_pool = 0;
    pool.offer_token.mint = ctx.accounts.offer_mint.key();
    pool.purchase_token = ctx.accounts.purchase_mint.key();
    pool.total_fee = 0;
    pool.private_raise = private;
    pool.bump = bump;
    Ok(())
}
