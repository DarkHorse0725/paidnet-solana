use anchor_lang::{prelude::*, solana_program::program_option::COption};
use anchor_spl::{
    token::{mint_to, transfer, MintTo, Transfer},
    token_2022::{transfer_checked, TransferChecked},
    token_interface,
};

use std::mem::size_of;

use crate::{AppState, Staker, AUTHORITY_SEED, REWARD_DENOMINATOR, STAKER_SEED, STAKE_VAULT_SEED};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    // mint address of stake token
    pub paid: Box<InterfaceAccount<'info, token_interface::Mint>>,
    #[account(
        mut,
        constraint = s_paid.mint_authority == COption::Some(authority.key()),
        constraint = s_paid.freeze_authority.is_none(),
    )]
    pub s_paid: Box<InterfaceAccount<'info, token_interface::Mint>>,

    // stake token account of staker
    #[account(mut)]
    pub user_paid_account: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,
    #[account(mut)]
    pub user_s_paid_account: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(seeds = [AUTHORITY_SEED, app_state.key().as_ref()], bump)]
    pub authority: AccountInfo<'info>,

    // stake token vault of stake program
    #[account(
      init_if_needed,
      payer = signer,
      token::mint = paid,
      token::authority = authority,
      seeds = [STAKE_VAULT_SEED],
      bump,
    )]
    pub stake_vault: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    // app state account of stake program
    #[account(mut)]
    pub app_state: Box<Account<'info, AppState>>,

    // staker account of user
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

impl<'info> Stake<'info> {
    pub fn transfer_checked_ctx(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_program: AccountInfo = self.token_program.to_account_info();
        let cpi_accounts: TransferChecked = TransferChecked {
            from: self.user_paid_account.to_account_info(),
            to: self.stake_vault.to_account_info(),
            authority: self.signer.to_account_info(),
            mint: self.paid.to_account_info(),
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
    fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.user_paid_account.to_account_info(),
                to: self.stake_vault.to_account_info(),
                authority: self.signer.to_account_info(),
            },
        )
    }
    fn to_mint_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: self.s_paid.to_account_info().clone(),
            to: self.user_s_paid_account.to_account_info().clone(),
            authority: self.authority.clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

/**
 * They can stake stake token
 * @param amount to stake
 */

pub fn stake_handler(ctx: Context<Stake>, amount: u64) -> Result<()> {
    let real_amount: u64 = amount - ctx.accounts.app_state.fuel_percentage as u64 * amount / REWARD_DENOMINATOR;
    if ctx.accounts.app_state.is_token2022 {
        transfer_checked(
            ctx.accounts.transfer_checked_ctx(),
            amount,
            ctx.accounts.paid.decimals,
        )?;
    } else {
        transfer(ctx.accounts.transfer_ctx(), amount)?;
    }
    let seeds: &[&[u8]; 3] = &[
        AUTHORITY_SEED,
        ctx.accounts.app_state.to_account_info().key.as_ref(),
        &[ctx.accounts.app_state.bump],
    ];
    let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];
    mint_to(ctx.accounts.to_mint_context().with_signer(signer_seeds), real_amount)?;
    let staker: &mut Box<Account<Staker>> = &mut ctx.accounts.staker;
    let app_state: &mut Box<Account<AppState>> = &mut ctx.accounts.app_state;
    app_state.total_staked += real_amount as u128;
    if staker.total_amount == 0 {
      app_state.staker_counts += 1;
      staker.user = ctx.accounts.signer.key();
    }
    staker.total_amount += real_amount;
    let now: i64 = Clock::get().unwrap().unix_timestamp;
    staker.last_update = now;
    Ok(())
}
