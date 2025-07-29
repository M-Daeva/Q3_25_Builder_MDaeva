use {
    crate::types::{AssetItem, Range},
    anchor_lang::prelude::*,
};

pub const SECONDS_PER_DAY: u32 = 24 * 3_600;
pub const SECONDS_PER_YEAR: u32 = 365 * SECONDS_PER_DAY;

pub const ROTATION_TIMEOUT: u32 = SECONDS_PER_DAY;
pub const ACCOUNT_REGISTRATION_FEE_AMOUNT: u64 = 10_000_000; // 10 $
pub const ACCOUNT_REGISTRATION_FEE_ASSET: Pubkey =
    Pubkey::from_str_const("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"); // mainnet USDC
pub const ACCOUNT_DATA_SIZE_MIN: u32 = 512;
pub const ACCOUNT_DATA_SIZE_MAX: u32 = 8_192;
pub const ACCOUNT_LIFETIME_MIN: u32 = 2 * SECONDS_PER_YEAR;
pub const ACCOUNT_LIFETIME_MAX: u32 = 100 * SECONDS_PER_YEAR;
pub const ACCOUNT_LIFETIME_MARGIN_BPS: u16 = 2_000; // 20 %

pub const SEED_BUMP: &[u8] = b"bump";
pub const SEED_COMMON_CONFIG: &[u8] = b"common_config";
pub const SEED_ACCOUNT_CONFIG: &[u8] = b"account_config";
pub const SEED_USER_COUNTER: &[u8] = b"user_counter";
pub const SEED_ADMIN_ROTATION_STATE: &[u8] = b"admin_rotation_state";

/// to store bumps for all app accounts
#[account]
#[derive(InitSpace)]
pub struct Bump {
    pub common_config: u8,
    pub account_config: u8,
    pub user_counter: u8,
    pub rotation_state: u8,
}

/// common program settings
#[account]
#[derive(InitSpace)]
pub struct CommonConfig {
    /// can update the config and execute priveled instructions
    pub admin: Pubkey,
    pub dex_adapter: Option<Pubkey>,
    pub is_paused: bool,
    pub rotation_timeout: u32,
}

/// account-related program settings
#[account]
#[derive(InitSpace)]
pub struct AccountConfig {
    pub registration_fee: AssetItem,
    pub data_size_range: Range,
    pub lifetime_range: Range,
    pub lifetime_margin_bps: u16,
}

/// for indexing
#[account]
#[derive(InitSpace, Default)]
pub struct UserCounter {
    pub last_user_id: u32,
}

/// to transfer ownership from one address to another in 2 steps (for security reasons) \
/// used both for app admin and user accounts
#[account]
#[derive(InitSpace)]
pub struct RotationState {
    pub new_owner: Option<Pubkey>,
    pub expiration_date: u64,
}

/// get by user: Pubkey
#[account]
#[derive(InitSpace)]
pub struct UserId {
    pub value: u32,
    pub account_bump: u8,
    pub rotation_state_bump: u8,
}

/// get by user_id: u32
#[account]
#[derive(InitSpace)]
pub struct UserAccount {
    /// for indexing
    pub id: u32,
    pub is_activated: bool,
    /// encrypted user data
    #[max_len(ACCOUNT_DATA_SIZE_MAX)]
    pub data: String,
    /// encryption nonce
    pub nonce: u64,
    /// allocated storage capacity
    pub max_size: u32,
    pub expiration_date: u64,
    /// total rent paid so far
    pub rent_paid: u64,
    /// rent consumed based on time/usage
    pub rent_used: u64,
}
