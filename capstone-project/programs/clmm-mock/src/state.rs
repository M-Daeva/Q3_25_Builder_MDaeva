use anchor_lang::prelude::*;

pub const SEED_POOL_STATE: &str = "pool_state";

#[account]
#[derive(InitSpace)]
pub struct PoolState {
    pub id: u8,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub price_ratio: u128, // Scaled by 1e9
}
