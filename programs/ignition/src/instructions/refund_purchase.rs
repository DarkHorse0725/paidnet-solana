use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::{error::ErrCode, Buyer, Pool};

#[derive(Accounts)]
pub struct RefundPurchase<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    // mint address of purchase token
    pub purchase_mint: Box<Account<'info, Mint>>,

    // purchase tokena account of investor
    #[account(mut)]
    pub user_purchase_token: Box<Account<'info, TokenAccount>>,

    // buyer account of investor
    #[account(
        mut,
        seeds = [b"buyer", pool.key().as_ref(), user.key().as_ref()],
        bump,
      )]
    pub buyer: Box<Account<'info, Buyer>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(seeds = [b"authority", pool.key().as_ref()], bump)]
    pub authority: AccountInfo<'info>,

    // purchase token vault of pool
    #[account(
        mut,
        seeds = [b"purchase-vault", pool.key().as_ref()],
        bump,
        token::mint = purchase_mint,
        token::authority = authority
    )]
    pub purchase_vault: Box<Account<'info, TokenAccount>>,

    // pool account
    #[account(mut, constraint = pool.purchase_token.mint == purchase_mint.key())]
    pub pool: Box<Account<'info, Pool>>,
    pub token_program: Program<'info, Token>,
}

impl<'info> RefundPurchase<'info> {
    fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.purchase_vault.to_account_info(),
                to: self.user_purchase_token.to_account_info(),
                authority: self.authority.to_account_info(),
            },
        )
    }
}

/**
 * Investors can withdraw their purchase token after tge date if pool was failed.
 * Callaborator had to cancel pool
 */
pub fn refund_purchase_handler(ctx: Context<RefundPurchase>) -> Result<()> {
    if !ctx.accounts.pool.emergency_cancelled {
        return err!(ErrCode::NotRefundable);
    }
    let now: i64 = Clock::get().unwrap().unix_timestamp;
    if now < ctx.accounts.pool.tge_date {
        return err!(ErrCode::NotRefundable);
    }
    if ctx.accounts.pool.claimable {
        return err!(ErrCode::NotRefundable);
    }
    if ctx.accounts.buyer.claimed_amount > 0 {
        return err!(ErrCode::NotRefundable);
    }
    if ctx.accounts.buyer.refunded {
        return err!(ErrCode::NotRefundable);
    }
    let amount: u64 = ctx.accounts.buyer.principal + ctx.accounts.buyer.fee;
    transfer(ctx.accounts.transfer_ctx(), amount)?;
    let buyer: &mut Box<Account<Buyer>> = &mut ctx.accounts.buyer;
    buyer.refunded = true;
    Ok(())
}
