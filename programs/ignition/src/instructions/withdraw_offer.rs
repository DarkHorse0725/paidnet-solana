use anchor_lang::prelude::*;
use anchor_spl::{
    token::{transfer, Transfer},
    token_2022::{transfer_checked, TransferChecked, ID},
    token_interface,
};

use crate::{Pool, error::ErrCode};

#[derive(Accounts)]
pub struct WithdrawOffer<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    // mint address of offer token
    pub offer_mint: Box<InterfaceAccount<'info, token_interface::Mint>>,

    // offer token account of owner
    #[account(mut)]
    pub owner_offer_token: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(seeds = [b"authority", pool.key().as_ref()], bump)]
    pub authority: AccountInfo<'info>,

    // offer token vault of pool
    #[account(
        seeds = [b"offer-vault", pool.key().as_ref()],
        bump,
        token::mint = offer_mint,
        token::authority = authority
    )]
    pub offer_vault: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    // pool account, signer must be owner of pool
    #[account(mut, has_one = owner, close = owner)]
    pub pool: Box<Account<'info, Pool>>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
}

impl<'info> WithdrawOffer<'info> {
    pub fn transfer_checked_ctx(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_program: AccountInfo = self.token_program.to_account_info();
        let cpi_accounts: TransferChecked = TransferChecked {
            from: self.offer_vault.to_account_info(),
            to: self.owner_offer_token.to_account_info(),
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
                to: self.owner_offer_token.to_account_info(),
                authority: self.authority.to_account_info(),
            },
        )
    }
}

/**
 * Collaborator can withdraw offer token after tge date if pool was failed
 * tranfer functions is implemented for both of token 2000 and token 2022
 */
pub fn withdraw_offer_handler(ctx: Context<WithdrawOffer>) -> Result<()> {
    let now: i64 = Clock::get().unwrap().unix_timestamp;
    if now < ctx.accounts.pool.tge_date {
        return err!(ErrCode::InvalidTime);
    }
    let amount: u64 = ctx.accounts.pool.total_funded_amount;
    let seeds: &[&[u8]; 3] = &[
        b"authority",
        ctx.accounts.pool.to_account_info().key.as_ref(),
        &[ctx.accounts.pool.bump],
    ];
    let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];
    if ctx.accounts.token_program.key() == ID {
        transfer_checked(
            ctx.accounts
                .transfer_checked_ctx()
                .with_signer(signer_seeds),
            amount,
            ctx.accounts.offer_mint.decimals,
        )?;
    } else {
        transfer(
            ctx.accounts.transfer_ctx().with_signer(signer_seeds),
            amount,
        )?;
    }
    Ok(())
}
