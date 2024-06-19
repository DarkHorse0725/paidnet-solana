use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";
pub const MIN_STAKE_AMOUNT: u64 = 10000 * (10 ^ 6);
pub const APP_STATE_SEED: &[u8] = b"app-state";
pub const REWARD_POT_SEED: &[u8] = b"reward-pot";
pub const AUTHORITY_SEED: &[u8] = b"authority";