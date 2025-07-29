use anchor_lang::prelude::*;

pub const SECONDS_PER_DAY: u32 = 24 * 3_600;
pub const ROTATION_TIMEOUT: u32 = SECONDS_PER_DAY;
pub const TOKEN_IN_WHITELIST_MAX_LEN: usize = 16;

/// to store bumps for all app accounts
#[account]
#[derive(InitSpace)]
pub struct Bump {
    pub config: u8,
    pub rotation_state: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Config {
    /// can update the config and execute priveled instructions
    pub admin: Pubkey,
    pub registry: Option<Pubkey>,
    pub is_paused: bool,
    pub rotation_timeout: u32,
    /// list of supported SPL/Token2022 tokens
    #[max_len(TOKEN_IN_WHITELIST_MAX_LEN)]
    pub token_in_whitelist: Vec<Pubkey>,
}

/// to transfer ownership from one address to another in 2 steps (for security reasons)
#[account]
#[derive(InitSpace)]
pub struct RotationState {
    pub new_owner: Option<Pubkey>,
    pub expiration_date: u64,
}
