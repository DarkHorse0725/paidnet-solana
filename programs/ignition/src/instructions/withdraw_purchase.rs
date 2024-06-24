use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};
use stake::cpi::accounts::FundReward;
use stake::cpi::fund_reward;
use stake::program::Stake;

use crate::{Pool, DENOMINATOR};

#[derive(Accounts)]
pub struct WithdrawPurchase<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    pub purchase_mint: Box<Account<'info, Mint>>,

    #[account(mut, token::mint = purchase_mint)]
    pub owner_purchase_token: Box<Account<'info, TokenAccount>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(seeds = [b"authority", pool.key().as_ref()], bump)]
    pub authority: AccountInfo<'info>,

    #[account(mut)]
    pub pool: Box<Account<'info, Pool>>,

    #[account(
        mut,
        token::mint = purchase_mint,
        token::authority = authority
    )]
    pub purchase_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub reward_pot: Box<Account<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
    pub stake_program: Program<'info, Stake>,
}

impl<'info> WithdrawPurchase<'info> {
    fn fund_ctx(&self) -> CpiContext<'_, '_, '_, 'info, FundReward<'info>> {
        CpiContext::new(
            self.stake_program.to_account_info(),
            FundReward {
                reward_mint: self.purchase_mint.to_account_info(),
                signer: self.authority.to_account_info(),
                token_account: self.purchase_vault.to_account_info(),
                reward_pot: self.reward_pot.to_account_info(),
                token_program: self.token_program.to_account_info(),
            },
        )
    }
    fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.purchase_vault.to_account_info(),
                to: self.owner_purchase_token.to_account_info(),
                authority: self.authority.to_account_info(),
            },
        )
    }
}

pub fn withdraw_purchase_handler(ctx: Context<WithdrawPurchase>) -> Result<()> {
    let project_fee: u64 = ctx.accounts.pool.total_collect_amount
        * ctx.accounts.pool.token_fee_percentage as u64
        / DENOMINATOR as u64;
    let total_fee: u64 = project_fee + ctx.accounts.pool.total_fee;
    let seeds: &[&[u8]; 3] = &[
        b"authority",
        ctx.accounts.pool.to_account_info().key.as_ref(),
        &[ctx.accounts.pool.bump],
    ];
    let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];
    fund_reward(ctx.accounts.fund_ctx().with_signer(signer_seeds), total_fee)?;
    let amount: u64 = ctx.accounts.pool.total_collect_amount - project_fee;
    transfer(
        ctx.accounts.transfer_ctx().with_signer(signer_seeds),
        amount,
    )?;
    Ok(())
}
