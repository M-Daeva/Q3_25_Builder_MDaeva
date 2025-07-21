use {anchor_lang::prelude::*, base::error::NftError};

pub enum ProgError {
    Cusom(CustomError),
    Nft(NftError),
}

#[error_code]
pub enum CustomError {
    #[msg("Incorrect sender!")]
    Unathorized,

    #[msg("Asset is not found!")]
    AssetIsNotFound,
}

impl From<CustomError> for ProgError {
    fn from(error: CustomError) -> Self {
        Self::Cusom(error)
    }
}

impl From<NftError> for ProgError {
    fn from(error: NftError) -> Self {
        Self::Nft(error)
    }
}
