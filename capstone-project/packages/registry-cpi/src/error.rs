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

    #[msg("Max data size is exceeded!")]
    MaxDataSizeIsExceeded,

    #[msg("Wrong user ID!")]
    WrongUserId,

    #[msg("Account can't be activated twice!")]
    ActivateAccountTwice,

    #[msg("Account isn't activated!")]
    AccountIsNotActivated,

    #[msg("Account can't be opened twice!")]
    OpenAccountTwice,

    #[msg("Account isn't opened!")]
    AccountIsNotOpened,

    #[msg("Nonce must be unique!")]
    BadNonce,
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
