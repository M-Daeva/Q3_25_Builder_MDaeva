use {anchor_lang::prelude::*, base::error::AuthError};

pub enum ProgError {
    Auth(AuthError),
    Cusom(CustomError),
}

#[error_code]
pub enum CustomError {
    #[msg("Parameters are not provided!")]
    NoParameters,

    #[msg("Insufficient SOL amount for account creation")]
    InsufficientSolAmount,

    #[msg("Insufficient USDC amount for account activation")]
    InsufficientUsdcAmount,

    #[msg("Account already activated")]
    AccountAlreadyActivated,
}

impl From<CustomError> for ProgError {
    fn from(error: CustomError) -> Self {
        Self::Cusom(error)
    }
}

impl From<AuthError> for ProgError {
    fn from(error: AuthError) -> Self {
        Self::Auth(error)
    }
}
