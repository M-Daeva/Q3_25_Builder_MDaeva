use anchor_lang::prelude::*;

pub const SECONDS_PER_DAY: u32 = 24 * 3_600;
pub const ROTATION_TIMEOUT: u32 = SECONDS_PER_DAY;
pub const TOKEN_IN_WHITELIST_MAX_LEN: usize = 16;

pub const MAINNET_USDC: Pubkey =
    Pubkey::from_str_const("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
pub const MAINNET_WSOL: Pubkey =
    Pubkey::from_str_const("So11111111111111111111111111111111111111112");
pub const MAINNET_WBTC: Pubkey =
    Pubkey::from_str_const("5XZw2LKTyrfvfiskJ78AMpackRjPcyCif1WhUsPDuVqQ");

pub const SEED_BUMP: &[u8] = b"bump";
pub const SEED_CONFIG: &[u8] = b"config";
pub const SEED_ADMIN_ROTATION_STATE: &[u8] = b"admin_rotation_state";

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
