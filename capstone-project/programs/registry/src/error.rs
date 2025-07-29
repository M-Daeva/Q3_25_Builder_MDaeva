use anchor_lang::prelude::*;

#[error_code]
pub enum ProgError {
    #[msg("Insufficient SOL amount for account creation")]
    InsufficientSolAmount,

    #[msg("Insufficient USDC amount for account activation")]
    InsufficientUsdcAmount,

    #[msg("Account already activated")]
    AccountAlreadyActivated,

    #[msg("Unauthorized program caller")]
    UnauthorizedCaller,
}
