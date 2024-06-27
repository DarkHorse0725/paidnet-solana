use anchor_lang::prelude::*;

use crate::Pool;

#[derive(Accounts)]
pub struct ToggleClaimable<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    // pool account, signer must be owner of pool
    #[account(mut, has_one = owner)]
    pub pool: Box<Account<'info, Pool>>,
}

/**
 * Collaborator toggle claimable of their pool
 */
pub fn toggle_claimable_handler(ctx: Context<ToggleClaimable>) -> Result<()> {
    let pool: &mut Box<Account<Pool>> = &mut ctx.accounts.pool;
    pool.claimable = !pool.claimable;
    Ok(())
}
