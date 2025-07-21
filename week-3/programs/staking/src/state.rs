use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub config_bump: u8,
    pub rewards_bump: u8,
    /// tokens per second for 1 staked nft
    pub rewards_rate: u8,
    pub max_stake: u64,
    pub collection: Pubkey,
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

// nft program
//
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Token {
    pub token_bump: u8,
    pub mint_bump: u8,
    pub id: u16,
    pub collection: Pubkey,
    pub mint: Pubkey,
    pub metadata: String,
}
