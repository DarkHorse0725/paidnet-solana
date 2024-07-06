use anchor_lang::prelude::*;
use anchor_spl::{
    token::{transfer, Transfer},
    token_2022::{transfer_checked, TransferChecked, ID},
    token_interface,
};

use crate::Pool;

#[derive(Accounts)]
pub struct FundOffer<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    pub offer_mint: Box<InterfaceAccount<'info, token_interface::Mint>>,

    #[account(mut)]
    pub owner_offer_token: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(seeds = [b"authority", pool.key().as_ref()], bump)]
    pub authority: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = owner,
        seeds = [b"offer-vault", pool.key().as_ref()],
        bump,
        token::mint = offer_mint,
        token::authority = authority
    )]
    pub offer_vault: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    #[account(mut, has_one = owner)]
    pub pool: Box<Account<'info, Pool>>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> FundOffer<'info> {
    pub fn transfer_checked_ctx(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_program: AccountInfo = self.token_program.to_account_info();
        let cpi_accounts: TransferChecked = TransferChecked {
            from: self.owner_offer_token.to_account_info(),
            to: self.offer_vault.to_account_info(),
            authority: self.owner.to_account_info(),
            mint: self.offer_mint.to_account_info(),
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
    fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.owner_offer_token.to_account_info(),
                to: self.offer_vault.to_account_info(),
                authority: self.owner.to_account_info(),
            },
        )
    }
}

/**
 * Collaborator must be fund offer token after creating pool
 * amount is total raise amount * rate
 * offer token might be spl token 2000 or spl token 2022
 * Collaborator must be disable token fee for fundraising
 * After presale, they can update fee
 */


pub fn fund_offer_handler(ctx: Context<FundOffer>, amount: u64) -> Result<()> {
    if ctx.accounts.token_program.key() == ID {
        transfer_checked(
            ctx.accounts.transfer_checked_ctx(),
            amount,
            ctx.accounts.offer_mint.decimals,
        )?;
    } else {
        transfer(ctx.accounts.transfer_ctx(), amount)?;
    }
    let pool: &mut Box<Account<Pool>> = &mut ctx.accounts.pool;
    pool.total_funded_amount += amount;
    pool.funded = true;
    Ok(())
}
