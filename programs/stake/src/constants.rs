use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";
pub const MIN_STAKE_AMOUNT: u64 = 10000 * (10 ^ 6);