use anchor_lang::prelude::*;

#[error_code]
pub enum ProgError {
    #[msg("Overflow!")]
    Overflow,
}
