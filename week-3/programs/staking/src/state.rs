use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub config_bump: u8,
    pub rewards_bump: u8,
    /// tokens per second for 1 staked nft
    pub rewards_rate: u8,
    pub max_stake: u64,
    pub nft_mint: Pubkey,
    pub rewards_mint: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub bump: u8,
    #[max_len(32)]
    pub tokens: Vec<u16>,
    pub updated_at: u64,
    pub rewards: u64,
}
