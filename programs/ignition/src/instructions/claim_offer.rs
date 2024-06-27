use anchor_lang::prelude::*;
use anchor_spl::{
    token::{transfer, Transfer},
    token_2022::{transfer_checked, TransferChecked},
    token_interface,
};

use crate::{error::ErrCode, Buyer, Pool};

#[derive(Accounts)]
pub struct ClaimOffer<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub offer_mint: Box<InterfaceAccount<'info, token_interface::Mint>>,

    #[account(mut)]
    pub user_offer_token: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(seeds = [b"authority", pool.key().as_ref()], bump)]
    pub authority: AccountInfo<'info>,

    #[account(
        seeds = [b"offer-vault", pool.key().as_ref()],
        bump,
        token::mint = offer_mint,
        token::authority = authority
    )]
    pub offer_vault: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    #[account(mut)]
    pub pool: Box<Account<'info, Pool>>,

    #[account(
      mut,
      seeds = [b"buyer", pool.key().as_ref(), user.key().as_ref()],
      bump,
    )]
    pub buyer: Box<Account<'info, Buyer>>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
}

impl<'info> ClaimOffer<'info> {
    pub fn transfer_checked_ctx(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_program: AccountInfo = self.token_program.to_account_info();
        let cpi_accounts: TransferChecked = TransferChecked {
            from: self.offer_vault.to_account_info(),
            to: self.user_offer_token.to_account_info(),
            authority: self.authority.to_account_info(),
            mint: self.offer_mint.to_account_info(),
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
    fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.offer_vault.to_account_info(),
                to: self.user_offer_token.to_account_info(),
                authority: self.authority.to_account_info(),
            },
        )
    }
}

/**
 *  Investors can claim offer token after claimable token after success pool
 *  Collaborator had to be set claimable after completed pool
 */

pub fn claim_offer_handler(ctx: Context<ClaimOffer>) -> Result<()> {
    if !ctx.accounts.pool.claimable {
        return err!(ErrCode::NotClaimable);
    }
    let now: i64 = Clock::get().unwrap().unix_timestamp;
    if now < ctx.accounts.pool.tge_date {
        return err!(ErrCode::NotClaimable);
    }
    let amount: u64 = ctx.accounts.pool.calculate_claimable_amount(
        ctx.accounts.buyer.total_amount,
        ctx.accounts.buyer.claimed_amount,
    );
    let seeds: &[&[u8]; 3] = &[
        b"authority",
        ctx.accounts.pool.to_account_info().key.as_ref(),
        &[ctx.accounts.pool.bump],
    ];
    let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];
    if ctx.accounts.pool.is_token22 {
        transfer_checked(
            ctx.accounts
                .transfer_checked_ctx()
                .with_signer(signer_seeds),
            amount,
            ctx.accounts.pool.offer_token.decimals,
        )?;
    } else {
        transfer(
            ctx.accounts.transfer_ctx().with_signer(signer_seeds),
            amount,
        )?;
    }
    Ok(())
}
