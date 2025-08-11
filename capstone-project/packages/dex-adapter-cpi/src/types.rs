use anchor_lang::prelude::*;

#[derive(AnchorSerialize)]
pub struct SwapRouterBaseInData {
    pub discriminator: [u8; 8],
    pub amount_in: u64,
    pub amount_out_minimum: u64,
}

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub struct RouteItem {
    pub amm_index: u16,
    pub token_out: Pubkey,
}
