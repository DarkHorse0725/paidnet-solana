use anchor_lang::prelude::*;

use crate::Pool;

#[derive(Accounts)]
pub struct FinalizePool<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut, has_one = owner)]
    pub pool: Box<Account<'info, Pool>>,
}

pub fn finalize_pool_handler(ctx: Context<FinalizePool>) -> Result<()> {
    let pool: &mut Box<Account<Pool>> = &mut ctx.accounts.pool;
    pool.claimable = true;
    Ok(())
}
