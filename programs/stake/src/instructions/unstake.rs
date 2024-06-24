use anchor_lang::prelude::*;
use anchor_spl::{
    token::{transfer, Transfer},
    token_2022::{transfer_checked, TransferChecked},
    token_interface,
};

use std::mem::size_of;

use crate::{error::ErrorCode, AppState, Staker, AUTHORITY_SEED, STAKER_SEED, STAKE_VAULT_SEED};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub stake_mint: Box<InterfaceAccount<'info, token_interface::Mint>>,

    #[account(mut)]
    pub user_stake_token: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(seeds = [AUTHORITY_SEED, app_state.key().as_ref()], bump)]
    pub authority: AccountInfo<'info>,

    #[account(
      init_if_needed,
      payer = signer,
      token::mint = stake_mint,
      token::authority = authority,
      seeds = [STAKE_VAULT_SEED],
      bump,
    )]
    pub stake_vault: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    #[account(mut)]
    pub app_state: Box<Account<'info, AppState>>,

    #[account(
      init_if_needed,
      payer = signer,
      space = size_of::<Staker>() + 8,
      seeds = [STAKER_SEED, signer.key().as_ref()],
      bump,
    )]
    pub staker: Box<Account<'info, Staker>>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Unstake<'info> {
    pub fn transfer_checked_ctx(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_program: AccountInfo = self.token_program.to_account_info();
        let cpi_accounts: TransferChecked = TransferChecked {
            from: self.stake_vault.to_account_info(),
            to: self.user_stake_token.to_account_info(),
            authority: self.authority.to_account_info(),
            mint: self.stake_mint.to_account_info(),
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
    fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.stake_vault.to_account_info(),
                to: self.user_stake_token.to_account_info(),
                authority: self.authority.to_account_info(),
            },
        )
    }
}

pub fn unstake_handler(ctx: Context<Unstake>, amount: u64) -> Result<()> {
    if amount > ctx.accounts.staker.total_amount {
        return err!(ErrorCode::InvalidUnstakeAmount);
    }
    let seeds: &[&[u8]; 3] = &[
        AUTHORITY_SEED,
        ctx.accounts.app_state.to_account_info().key.as_ref(),
        &[ctx.accounts.app_state.bump],
    ];
    let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];
    if ctx.accounts.app_state.stake_token.is_token2 {
        transfer_checked(
            ctx.accounts
                .transfer_checked_ctx()
                .with_signer(signer_seeds),
            amount,
            ctx.accounts.app_state.stake_token.decimals,
        )?;
    } else {
        transfer(
            ctx.accounts.transfer_ctx().with_signer(signer_seeds),
            amount,
        )?;
    }
    let staker: &mut Box<Account<Staker>> = &mut ctx.accounts.staker;
    staker.total_amount -= amount;
    let now: i64 = Clock::get().unwrap().unix_timestamp;
    staker.last_update = now;
    let app_state: &mut Box<Account<AppState>> = &mut ctx.accounts.app_state;
    app_state.total_staked -= amount as u128;
    Ok(())
}
