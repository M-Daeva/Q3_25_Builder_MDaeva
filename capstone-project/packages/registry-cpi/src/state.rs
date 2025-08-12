use {
    crate::types::{AssetItem, Range},
    anchor_lang::prelude::*,
};

pub const SECONDS_PER_DAY: u32 = 24 * 3_600;
pub const SECONDS_PER_YEAR: u32 = 365 * SECONDS_PER_DAY;

pub const CLOCK_TIME_MIN: u64 = 1750000000;
pub const MAINNET_ADMIN: Pubkey =
    Pubkey::from_str_const("AH9JvTDAiQy2zAuFfzteNyUrW5DYoTsTLoeNjXrxTTSt");

pub const ROTATION_TIMEOUT: u32 = SECONDS_PER_DAY;
pub const ACCOUNT_REGISTRATION_FEE_AMOUNT: u64 = 10_000_000; // 10 $
pub const ACCOUNT_REGISTRATION_FEE_ASSET: Pubkey =
    Pubkey::from_str_const("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"); // mainnet USDC
pub const ACCOUNT_DATA_SIZE_MIN: u32 = 100;
pub const ACCOUNT_DATA_SIZE_MAX: u32 = 100_000;

pub const SEED_BUMP: &str = "bump";
pub const SEED_CONFIG: &str = "config";
pub const SEED_USER_COUNTER: &str = "user_counter";
pub const SEED_ADMIN_ROTATION_STATE: &str = "admin_rotation_state";

pub const SEED_USER_ID: &str = "user_id";
pub const SEED_USER_ACCOUNT: &str = "user_account";
pub const SEED_USER_ROTATION_STATE: &str = "user_rotation_state";

/// to store bumps for all app accounts
#[account]
#[derive(InitSpace)]
pub struct Bump {
    pub config: u8,
    pub user_counter: u8,
    pub rotation_state: u8,
}

#[account]
#[derive(InitSpace, PartialEq, Debug)]
pub struct Config {
    /// can update the config and execute priveled instructions
    pub admin: Pubkey,
    pub is_paused: bool,
    pub rotation_timeout: u32,
    pub registration_fee: AssetItem,
    pub data_size_range: Range,
}

/// for indexing
#[account]
#[derive(InitSpace, Default, PartialEq, Debug)]
pub struct UserCounter {
    pub last_user_id: u32,
}

/// to transfer ownership from one address to another in 2 steps (for security reasons) \
/// used both for app admin and user accounts
#[account]
#[derive(InitSpace, PartialEq, Debug)]
pub struct RotationState {
    pub owner: Pubkey,
    pub new_owner: Option<Pubkey>,
    pub expiration_date: u64,
}

/// get by user: Pubkey
#[account]
#[derive(InitSpace, PartialEq, Debug)]
pub struct UserId {
    pub id: u32,
    pub is_open: bool,
    pub is_activated: bool,
    pub account_bump: u8,
    pub rotation_state_bump: u8,
}

/// get by user_id: u32
#[account]
#[derive(PartialEq, Debug)]
pub struct UserAccount {
    /// encrypted user data
    pub data: String,
    /// encryption nonce
    pub nonce: u64,
    /// allocated storage capacity
    pub max_size: u32,
}

impl UserAccount {
    pub fn get_space(max_size: u32) -> usize {
        8 +   // discriminator
        4 + max_size as usize + // data (String: 4 bytes length + content)
        8 +   // nonce (u64)
        4 // max_size (u32)
    }
}
