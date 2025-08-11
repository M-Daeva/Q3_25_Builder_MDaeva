use {crate::types::RouteItem, anchor_lang::prelude::*};

pub const SECONDS_PER_DAY: u32 = 24 * 3_600;
pub const ROTATION_TIMEOUT: u32 = SECONDS_PER_DAY;
pub const TOKEN_IN_WHITELIST_MAX_LEN: usize = 16;
pub const ROUTE_MAX_LEN: usize = 4;

pub const SEED_BUMP: &str = "bump";
pub const SEED_CONFIG: &str = "config";
pub const SEED_ADMIN_ROTATION_STATE: &str = "admin_rotation_state";
pub const SEED_ROUTE: &str = "route";

// anchor can't resolve name conflicts so we need to rename accounts like Bump -> DaBump

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
}

/// to transfer ownership from one address to another in 2 steps (for security reasons)
#[account]
#[derive(InitSpace, PartialEq, Debug)]
pub struct RotationState {
    pub owner: Pubkey,
    pub new_owner: Option<Pubkey>,
    pub expiration_date: u64,
}

#[account]
#[derive(InitSpace, PartialEq, Debug)]
pub struct Route {
    #[max_len(ROUTE_MAX_LEN)]
    pub value: Vec<RouteItem>,
}
