use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub struct SwapSpec {
    pub token_out: Pubkey,
    /// e.g., 1 for 1/11
    pub ratio_numerator: u64,
    /// e.g., 11 for 1/11
    pub ratio_denominator: u64,
    pub min_amount_out: u64,
    /// where to send the output
    pub recipient: Pubkey,
    /// optional program to forward to
    pub forward_to_program: Option<Pubkey>,
}

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
