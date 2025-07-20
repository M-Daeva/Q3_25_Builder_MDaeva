use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug, PartialEq)]
pub struct PoolConfig {
    pub config_bump: u8,
    pub balance_bump: u8,
    pub lp_bump: u8,
    pub id: u64,
    pub authority: Option<Pubkey>,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub mint_lp: Pubkey,
    pub fee_bps: u16,
    pub is_locked: bool,
}

#[account]
#[derive(InitSpace, Debug, PartialEq, Default)]
pub struct PoolBalance {
    pub mint_x_amount: u64,
    pub mint_y_amount: u64,
    pub mint_lp_amount: u64,
}
