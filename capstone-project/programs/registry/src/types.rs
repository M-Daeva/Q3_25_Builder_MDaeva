use anchor_lang::prelude::*;

pub struct AssetItem {
    pub amount: u64,
    pub asset: Pubkey,
}

pub struct Range {
    pub min: u32,
    pub max: u32,
}
