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
}
