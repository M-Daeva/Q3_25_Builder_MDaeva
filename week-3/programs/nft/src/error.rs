use anchor_lang::prelude::*;

#[error_code]
pub enum ProgError {
    #[msg("NFT isn't found!")]
    NftIsNotFound,

    #[msg("Collection isn't found!")]
    CollectionIsNotFound,

    #[msg("Empty token list!")]
    EmptyTokenList,

    #[msg("Empty collection list!")]
    EmptyCollectionList,

    #[msg("NFT list has duplications!")]
    NftDuplication,

    #[msg("Collection already exists!")]
    CollectionDuplication,

    #[msg("Incorrect token list!")]
    IncorrectTokenList,

    #[msg("Incorrect token list!")]
    IncorrectCollectionList,

    #[msg("Max token amount per tx is exceeded!")]
    ExceededTokenLimit,

    #[msg("Collection isn't added!")]
    CollectionIsNotAdded,
}
