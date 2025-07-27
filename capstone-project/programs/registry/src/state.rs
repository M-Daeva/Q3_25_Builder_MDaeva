use {
    crate::types::{AssetItem, Range},
    anchor_lang::prelude::*,
};

// TODO: add default values
// dex_adapter:
// rotation_timeout:
// account_registration_fee:
// account_data_size_range:
// account_lifetime_range:
// account_lifetime_margin_bps:

/// to store bumps for all app accounts
pub struct Bump {
    pub common_config: u8,
    pub account_config: u8,
    pub user_counter: u8,
    pub revenue: u8,
    pub rotation_state: u8,
}

/// common program settings
pub struct CommonConfig {
    /// can update the config and execute priveled instructions
    pub admin: Pubkey,
    pub dex_adapter: Option<Pubkey>,
    pub is_paused: bool,
    pub rotation_timeout: u32,
}

/// account-related program settings
pub struct AccountConfig {
    pub registration_fee: AssetItem,
    pub data_size_range: Range,
    pub lifetime_range: Range,
    pub lifetime_margin_bps: u16,
}

/// for pagination
pub struct UserCounter {
    pub last_user_id: u32,
}

/// earned revenue balance
pub struct Revenue {
    pub amount: u64,
}

/// to transfer ownership from one address to another in 2 steps (for security reasons) \
/// used both for app admin and user accounts
pub struct RotationState {
    pub new_owner: Option<Pubkey>,
    pub expiration_date: u64,
}

/// get by user: Pubkey
pub struct UserId {
    pub value: u32,
    pub account_bump: u8,
    pub rotation_state_bump: u8,
}

/// get by user_id: u32
pub struct UserAccount {
    /// for pagination
    pub id: u32,
    pub is_activated: bool,
    /// encrypted user data
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
