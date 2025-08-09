use {crate::types::RouteItem, anchor_lang::prelude::*};

pub const SECONDS_PER_DAY: u32 = 24 * 3_600;
pub const ROTATION_TIMEOUT: u32 = SECONDS_PER_DAY;
pub const TOKEN_IN_WHITELIST_MAX_LEN: usize = 16;
pub const ROUTE_MAX_LEN: usize = 3;

pub const MAINNET_USDC: Pubkey =
    Pubkey::from_str_const("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
pub const MAINNET_WSOL: Pubkey =
    Pubkey::from_str_const("So11111111111111111111111111111111111111112");
pub const MAINNET_WBTC: Pubkey =
    Pubkey::from_str_const("5XZw2LKTyrfvfiskJ78AMpackRjPcyCif1WhUsPDuVqQ");

pub const SEED_BUMP: &str = "bump";
pub const SEED_CONFIG: &str = "config";
pub const SEED_ADMIN_ROTATION_STATE: &str = "admin_rotation_state";
pub const SEED_ROUTE: &str = "route";

/// to store bumps for all app accounts
#[account]
#[derive(InitSpace)]
pub struct DaBump {
    pub config: u8,
    pub rotation_state: u8,
}

#[account]
#[derive(InitSpace, PartialEq, Debug)]
pub struct DaConfig {
    /// can update the config and execute priveled instructions
    pub admin: Pubkey,
    pub dex: Pubkey,
    pub registry: Option<Pubkey>,
    pub is_paused: bool,
    pub rotation_timeout: u32,
    // TODO: can we use route instead?
    /// list of supported SPL/Token2022 tokens
    #[max_len(TOKEN_IN_WHITELIST_MAX_LEN)]
    pub token_in_whitelist: Vec<Pubkey>,
}

/// to transfer ownership from one address to another in 2 steps (for security reasons)
#[account]
#[derive(InitSpace, PartialEq, Debug)]
pub struct RotationState {
    pub new_owner: Option<Pubkey>,
    pub expiration_date: u64,
}

#[account]
#[derive(InitSpace, PartialEq, Debug)]
pub struct Route {
    #[max_len(ROUTE_MAX_LEN + 1)] // 1 for 1st symbol
    pub value: Vec<RouteItem>,
}
