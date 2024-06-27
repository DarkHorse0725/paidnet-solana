use anchor_lang::prelude::*;

use crate::Pool;

#[derive(Accounts)]
pub struct EmergencyCancel<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    // pool account, signer must be owner of pool
    #[account(mut, has_one = owner)]
    pub pool: Box<Account<'info, Pool>>,
}

/**
 * Collaborator can cancel pool
 */
pub fn emergency_cancel_handler(ctx: Context<EmergencyCancel>) -> Result<()> {
    let pool: &mut Box<Account<Pool>> = &mut ctx.accounts.pool;
    pool.emergency_cancelled = true;
    Ok(())
}
