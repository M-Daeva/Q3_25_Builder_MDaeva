use anchor_lang::prelude::*;

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
