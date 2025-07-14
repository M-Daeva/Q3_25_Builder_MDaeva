use anchor_lang::prelude::*;

// TODO: use amm errors
#[error_code]
pub enum ProgError {
    #[msg("No liquidity is provided")]
    NoLiquidity,

    #[msg("The mint isn't supported")]
    WrongMint,
}
