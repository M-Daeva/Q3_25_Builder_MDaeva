use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Collection {
    pub bump: u8,
    pub id: u8,
    pub next_token_id: u16,
    pub creator: Pubkey,
    pub address: Pubkey,
    #[max_len(32)]
    pub metadata: String,
}

#[account]
#[derive(InitSpace)]
pub struct Token {
    pub token_bump: u8,
    pub mint_bump: u8,
    pub id: u16,
    pub collection: Pubkey,
    #[max_len(32)]
    pub metadata: String,
}
