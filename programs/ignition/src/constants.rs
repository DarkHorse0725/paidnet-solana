use anchor_lang::prelude::*;

#[constant]
pub const DENOMINATOR: u16 = 10000;
pub const STAKE_LIMIT: u64 = 500;
pub const MAX_TGE_DATE_ADJUSTMENT_ATTEMPTS: u16 = 2;