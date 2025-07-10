use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub bump: u8,
    /// each escrow id for given maker must be unique
    pub id: u8,
    pub maker: TraderInfo,
    pub taker: TraderInfo,
}

#[derive(InitSpace, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TraderInfo {
    pub trader: Pubkey,
    pub amount: u64,
    pub mint: Pubkey,
}
