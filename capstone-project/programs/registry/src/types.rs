use anchor_lang::prelude::*;

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone)]
pub struct AssetItem {
    pub amount: u64,
    pub asset: Pubkey,
}

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Range {
    pub min: u32,
    pub max: u32,
}
