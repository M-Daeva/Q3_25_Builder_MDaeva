#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;
pub mod types;

use {
    instructions::init::*,
    types::{AssetItem, Range},
};

declare_id!("3XEw4Ta4PU5NMET3xJhc71yagoB85awhTzzcNdFbAyBt");

// #[program]
pub mod registry {
    use super::*;

    pub fn init(
        dex_adapter: Option<Pubkey>,
        rotation_timeout: Option<u32>,
        token_whitelist: Option<Vec<Pubkey>>,
        account_registration_fee: Option<AssetItem>,
        account_data_size_range: Option<Range>,
        account_lifetime_range: Option<Range>,
        account_lifetime_margin_bps: Option<u16>,
    ) -> Result<()> {
        unimplemented!()
    }

    pub fn update_common_config(
        admin: Option<Pubkey>,
        dex_adapter: Option<Pubkey>,
        is_paused: Option<bool>,
        rotation_timeout: Option<u32>,
        token_whitelist: Option<Vec<Pubkey>>,
    ) -> Result<()> {
        unimplemented!()
    }

    pub fn update_account_config(
        registration_fee: Option<AssetItem>,
        data_size_range: Option<Range>,
        lifetime_range: Option<Range>,
        lifetime_margin_bps: Option<u16>,
    ) -> Result<()> {
        unimplemented!()
    }

    pub fn confirm_admin_rotation() -> Result<()> {
        unimplemented!()
    }

    pub fn withdraw_revenue(amount: Option<u64>, recipient: Option<Pubkey>) -> Result<()> {
        unimplemented!()
    }

    /// creates user PDA account accepting rent exempt in SOL
    pub fn create_account(max_data_size: u32, lifetime: u32) -> Result<()> {
        unimplemented!()
    }

    pub fn close_account() -> Result<()> {
        unimplemented!()
    }

    /// activates account with fee asset payment
    pub fn activate_account() -> Result<()> {
        unimplemented!()
    }

    /// automatic activation when fee asset is received from DEX Adapter
    pub fn receive_payment(user_account: Pubkey) -> Result<()> {
        unimplemented!()
    }

    /// to know rent exempt for PDA creation before the action
    pub fn simulate_rent_cost(
        data: String,
        lifetime: u64,
        margin_bps: Option<u16>, // overrides config value
    ) -> Result<u64> {
        unimplemented!()
    }

    /// to increase/decrease rent when more/less data/lifetime is required
    pub fn update_storage(max_size: Option<u32>, expiration_date: Option<u64>) -> Result<()> {
        unimplemented!()
    }

    pub fn write_data(data: String, nonce: u64) -> Result<()> {
        unimplemented!()
    }

    pub fn request_account_rotation(new_owner: Pubkey) -> Result<()> {
        unimplemented!()
    }

    /// updates address - id pair
    pub fn confirm_account_rotation() -> Result<()> {
        unimplemented!()
    }
}
