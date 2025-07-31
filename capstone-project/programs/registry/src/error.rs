use {anchor_lang::prelude::*, base::error::AuthError};

pub enum ProgError {
    Auth(AuthError),
    Cusom(CustomError),
}

#[error_code]
pub enum CustomError {
    #[msg("Parameters are not provided!")]
    NoParameters,

    #[msg("Wrong asset type!")]
    WrongAssetType,

    #[msg("Zero amount to send!")]
    ZeroAmount,

    #[msg("Exceeded available asset amount!")]
    ExceededAvailableAssetAmount,

    #[msg("The contract is temporary paused!")]
    ContractIsPaused,

    #[msg("Max data size is out of range!")]
    MaxDataSizeIsOutOfRange,

    #[msg("Wrong user ID!")]
    WrongUserId,
    //
    // #[msg("Insufficient SOL amount for account creation")]
    // InsufficientSolAmount,

    // #[msg("Insufficient USDC amount for account activation")]
    // InsufficientUsdcAmount,

    // #[msg("Account already activated")]
    // AccountAlreadyActivated,
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
