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
 * @param uints Array of pool informaton includes:
 *
 */

pub fn create_pool_handler(
    ctx: Context<CreatePool>,
    uints: [u64; 17],
    offer_decimals: u8,
    purchase_decimals: u8,
    is_token22: bool,
    bump: u8,
) -> Result<()> {
    let pool: &mut Box<Account<Pool>> = &mut ctx.accounts.pool;
    if uints[2] as u16 > DENOMINATOR {
        return err!(ErrCode::InvalidTokenFeePercentage);
    }
    // early pool proportion
    if uints[5] as u16 > DENOMINATOR {
        return err!(ErrCode::InvalidTokenFeePercentage);
    }
    // open pool proportion
    if uints[6] as u16 > DENOMINATOR {
        return err!(ErrCode::InvalidTokenFeePercentage);
    }
    if uints[8] > uints[9] {
        return err!(ErrCode::InvalidTime);
    }
    if uints[9] > uints[10] {
        return err!(ErrCode::InvalidTime);
    }
    if uints[10] > uints[12] {
        return err!(ErrCode::InvalidTime);
    }
    pool.max_buy_for_kyc_user = uints[0];
    pool.max_buy_for_not_kyc_user = uints[1];
    pool.token_fee_percentage = uints[2] as u16;
    pool.early_participant_fee = uints[3] as u16;
    pool.open_participant_fee = uints[4] as u16;
    pool.early_pool_proportion = uints[5] as u16;
    pool.open_pool_proportion = uints[6] as u16;
    pool.total_raise_amount = uints[7];
    pool.early_start = uints[8] as i64;
    pool.early_end = uints[9] as i64;
    pool.open_start = uints[9] as i64;
    pool.open_end = uints[10] as i64;

    pool.offer_token.rate = uints[11];
    pool.tge_date = uints[12] as i64;
    pool.tge_percentage = uints[13] as u16;
    pool.vesting_cliff = uints[14] as i64;
    pool.vesting_frequency = uints[15] as i64;
    pool.number_of_vesting = uints[16] as u16;
    pool.owner = ctx.accounts.creator.key();
    pool.total_collect_amount = 0;
    pool.total_sold = 0;
    pool.collect_in_early_pool = 0;
    pool.sold_in_early_pool = 0;
    pool.offer_token.mint = ctx.accounts.offer_mint.key();
    pool.offer_token.decimals = offer_decimals;
    pool.purchase_token.mint = ctx.accounts.purchase_mint.key();
    pool.purchase_token.decimals = purchase_decimals;
    pool.is_token22 = is_token22;
    pool.bump = bump;
    Ok(())
}
