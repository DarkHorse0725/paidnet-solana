use anchor_lang::prelude::*;
use anchor_spl::{
    token::{transfer, Mint, Token, TokenAccount, Transfer},
    token_interface,
};
use stake::{Staker, MIN_STAKE_AMOUNT};

use crate::{error::ErrCode, Buyer, Pool, DENOMINATOR};
use std::mem::size_of;

#[derive(Accounts)]
pub struct BuyInEarlyPool<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    // purchase token mint address
    pub purchase_mint: Box<Account<'info, Mint>>,
    pub offer_mint: Box<InterfaceAccount<'info, token_interface::Mint>>,

    // user purchase token account
    #[account(mut, token::mint = purchase_mint)]
    pub user_purchase_token: Box<Account<'info, TokenAccount>>,

    // staker account from stake program
    #[account(constraint = staker.user == signer.key())]
    pub staker: Box<Account<'info, Staker>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(seeds = [b"authority", pool.key().as_ref()], bump)]
    pub authority: AccountInfo<'info>,

    // buyer account of this pool
    #[account(
      init_if_needed,
      payer = signer,
      space = size_of::<Buyer>() + 8,
      seeds = [b"buyer", pool.key().as_ref(), signer.key().as_ref()],
      bump,
    )]
    pub buyer: Box<Account<'info, Buyer>>,

    #[account(
        init_if_needed,
        payer = signer,
        seeds = [b"purchase-vault", pool.key().as_ref()],
        bump,
        token::mint = purchase_mint,
        token::authority = authority
    )]
    pub purchase_vault: Box<Account<'info, TokenAccount>>,

    // pool account, purchase mint address must be same with the address of purchase token in pool account
    #[account(mut, constraint = pool.purchase_token == purchase_mint.key())]
    pub pool: Box<Account<'info, Pool>>,
    pub token_program: Program<'info, Token>,
    pub token_program_offer: Interface<'info, token_interface::TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> BuyInEarlyPool<'info> {
    fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.user_purchase_token.to_account_info(),
                to: self.purchase_vault.to_account_info(),
                authority: self.signer.to_account_info(),
            },
        )
    }
}

/**
 * Investor can buy token in early pool
 * They must be stake minimum stake amount to joing early pool
 * @param amount is purchase token amount
 * purchse token is stable tokens like usdt or usdc
 * Investors has limit to buy token. limit is based on purchase token
 */

pub fn buy_in_early_pool_handler(ctx: Context<BuyInEarlyPool>, amount: u64) -> Result<()> {
    let now: i64 = Clock::get().unwrap().unix_timestamp;
    if now < ctx.accounts.pool.early_start || now > ctx.accounts.pool.early_end {
        return err!(ErrCode::InvalidTime);
    }
    if !ctx.accounts.pool.funded {
        return err!(ErrCode::NotFunded);
    }
    let fee_amount: u64 =
        amount / DENOMINATOR as u64 * ctx.accounts.pool.early_participant_fee as u64;
    let total_amount: u64 = amount - fee_amount + ctx.accounts.buyer.total_amount;
    if total_amount > ctx.accounts.pool.max_buy_in_early_pool {
        return err!(ErrCode::ExceedMaxPurchaseAmountForEarlyAccess);
    }

    if ctx.accounts.staker.total_amount < MIN_STAKE_AMOUNT {
        return err!(ErrCode::NotEnoughStaker);
    }
    transfer(ctx.accounts.transfer_ctx(), amount)?;
    let buyer: &mut Box<Account<Buyer>> = &mut ctx.accounts.buyer;
    buyer.principal += amount - fee_amount;
    buyer.purchase_in_early_pool += amount - fee_amount;
    buyer.fee += fee_amount;
    let offer_amount: u64 = ctx.accounts.pool.calculate_offer_amount(
        amount - fee_amount,
        ctx.accounts.purchase_mint.decimals,
        ctx.accounts.offer_mint.decimals,
    );
    buyer.total_amount += offer_amount;
    let pool: &mut Box<Account<Pool>> = &mut ctx.accounts.pool;
    pool.total_collect_amount += amount - fee_amount;
    pool.total_sold += offer_amount;
    pool.collect_in_early_pool += amount - fee_amount;
    pool.sold_in_early_pool += offer_amount;
    pool.total_fee += fee_amount;
    Ok(())
}
