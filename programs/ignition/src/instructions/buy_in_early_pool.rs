use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::{error::ErrCode, Buyer, Pool};
use std::mem::size_of;

#[derive(Accounts)]
pub struct BuyInEarlyPool<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub purchase_mint: Box<Account<'info, Mint>>,

    #[account(mut, token::mint = purchase_mint)]
    pub user_purchase_token: Box<Account<'info, TokenAccount>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(seeds = [b"authority", pool.key().as_ref()], bump)]
    pub authority: AccountInfo<'info>,

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
        rent_exempt = enforce,
        token::mint = purchase_mint,
        token::authority = authority
    )]
    pub purchase_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = pool.purchase_token.mint == purchase_mint.key())]
    pub pool: Box<Account<'info, Pool>>,
    pub token_program: Program<'info, Token>,
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

pub fn buy_in_early_pool_handler(ctx: Context<BuyInEarlyPool>, amount: u64) -> Result<()> {
    let now: i64 = Clock::get().unwrap().unix_timestamp;
    if now < ctx.accounts.pool.early_start || now > ctx.accounts.pool.early_end {
        return err!(ErrCode::InvalidTime);
    }
    if !ctx.accounts.pool.funded {
        return err!(ErrCode::NotFunded);
    }
    let total_amount: u64 = ctx.accounts.buyer.total_amount.checked_add(amount).ok_or(ErrCode::CalculationFailure)?;
    if  total_amount > ctx.accounts.pool.max_buy_in_early_pool {
        return err!(ErrCode::ExceedMaxPurchaseAmountForEarlyAccess);
    }
    
    transfer(ctx.accounts.transfer_ctx(), amount)?;
    Ok(())
}
