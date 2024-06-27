use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct FundReward<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    // mint address of reward token
    pub reward_mint: Account<'info, Mint>,

    // token account of mint
    #[account(mut, token::mint = reward_mint)]
    pub token_account: Account<'info, TokenAccount>,

    // reward pot of stake program
    #[account(
      mut,
      token::mint = reward_mint,
    )]
    pub reward_pot: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> FundReward<'info> {
    fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.token_account.to_account_info(),
                to: self.reward_pot.to_account_info(),
                authority: self.signer.to_account_info(),
            },
        )
    }
}

/**
 * This funcition will be used for cpi
 */
pub fn fund_reward_handler(ctx: Context<FundReward>, amount: u64) -> Result<()> {
    token::transfer(ctx.accounts.transfer_ctx(), amount)?;
    Ok(())
}
