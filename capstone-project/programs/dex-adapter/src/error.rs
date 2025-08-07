use anchor_lang::prelude::*;

#[error_code]
pub enum ProgError {
    #[msg("Swap slippage exceeded maximum allowed")]
    SlippageExceeded,

    #[msg("Invalid swap ratio configuration")]
    InvalidSwapRatio,

    #[msg("DEX program call failed")]
    DexCallFailed,

    #[msg("Token forwarding failed")]
    ForwardingFailed,

    #[msg("Contract is paused")]
    ContractPaused,

    #[msg("Route must contain at least 2 tokens")]
    InvalidRouteLength,

    #[msg("Amount must be greater than 0")]
    InvalidAmount,

    #[msg("Invalid token account")]
    InvalidTokenAccount,

    #[msg("Invalid number of remaining accounts")]
    InvalidRemainingAccounts,

    #[msg("No output tokens received from swap")]
    NoOutputTokens,
}
