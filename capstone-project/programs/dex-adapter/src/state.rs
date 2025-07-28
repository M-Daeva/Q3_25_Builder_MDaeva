use anchor_lang::prelude::*;

// TODO: add default values
// registry:
// rotation_timeout:
// token_in_whitelist:

/// to store bumps for all app accounts
pub struct Bump {
    pub config: u8,
    pub rotation_state: u8,
}

pub struct Config {
    /// can update the config and execute priveled instructions
    pub admin: Pubkey,
    pub registry: Option<Pubkey>,
    pub is_paused: bool,
    pub rotation_timeout: u32,
    /// list of supported SPL/Token2022 tokens
    pub token_in_whitelist: Vec<Pubkey>,
}

/// to transfer ownership from one address to another in 2 steps (for security reasons)
pub struct RotationState {
    pub new_owner: Option<Pubkey>,
    pub expiration_date: u64,
}
