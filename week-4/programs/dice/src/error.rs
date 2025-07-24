use anchor_lang::prelude::*;

#[error_code]
pub enum DiceError {
    #[msg("Incorrect Ed25519 program!")]
    Ed25519Program,

    #[msg("Ed25519 accounts incorrent amount!")]
    Ed25519Accounts,

    #[msg("Ed25519 signatures incorrent amount!")]
    Ed25519Length,

    #[msg("Ed25519 incorrent header!")]
    Ed25519Header,

    #[msg("Ed25519 incorrent pubkey!")]
    Ed25519Pubkey,

    #[msg("Ed25519 incorrent signature!")]
    Ed25519Signature,

    #[msg("Overflow!")]
    Overflow,
}
