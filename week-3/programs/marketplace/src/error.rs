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

    #[msg("Fee is too big!")]
    FeeIsTooBig,

    #[msg("Asset list is emty!")]
    EmptyAssetList,

    #[msg("Asset already exists!")]
    AssetDuplication,
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
