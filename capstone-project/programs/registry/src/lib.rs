#![allow(unexpected_cfgs)]

pub mod instructions;

use {
    anchor_lang::prelude::*,
    instructions::{
        activate_account::*, close_account::*, confirm_account_rotation::*,
        confirm_admin_rotation::*, create_account::*, init::*, reopen_account::*,
        request_account_rotation::*, update_config::*, withdraw_revenue::*, write_data::*,
    },
    registry_cpi::types::{AssetItem, Range},
};

// IDL builder doesn't see ID from cpi package, we need to duplicate it here
declare_id!("3RVBZDA6dgcjkGJtXRJxLvLh5g8qY6QwGHoribyKPN2E");

#[program]
pub mod registry {
    use super::*;

    pub fn init(
        ctx: Context<Init>,
        rotation_timeout: Option<u32>,
        account_registration_fee: Option<AssetItem>,
        account_data_size_range: Option<Range>,
    ) -> Result<()> {
        ctx.accounts.init(
            ctx.bumps,
            rotation_timeout,
            account_registration_fee,
            account_data_size_range,
        )
    }

    pub fn update_config(
        ctx: Context<UpdateConfig>,
        admin: Option<Pubkey>,
        is_paused: Option<bool>,
        rotation_timeout: Option<u32>,
        registration_fee: Option<AssetItem>,
        data_size_range: Option<Range>,
    ) -> Result<()> {
        ctx.accounts.update_config(
            admin,
            is_paused,
            rotation_timeout,
            registration_fee,
            data_size_range,
        )
    }

    pub fn confirm_admin_rotation(ctx: Context<ConfirmAdminRotation>) -> Result<()> {
        ctx.accounts.confirm_admin_rotation()
    }

    pub fn withdraw_revenue(ctx: Context<WithdrawRevenue>, amount: Option<u64>) -> Result<()> {
        ctx.accounts.withdraw_revenue(amount)
    }

    /// creates user PDA account taking rent exempt in SOL
    pub fn create_account(
        ctx: Context<CreateAccount>,
        max_data_size: u32,
        expected_user_id: u32,
    ) -> Result<()> {
        ctx.accounts
            .create_account(ctx.bumps, max_data_size, expected_user_id)
    }

    /// 1st step to to change allocated data space or just to redeem rent
    pub fn close_account(ctx: Context<CloseAccount>) -> Result<()> {
        ctx.accounts.close_account()
    }

    /// 2nd step to to change allocated data space
    pub fn reopen_account(ctx: Context<ReopenAccount>, max_data_size: u32) -> Result<()> {
        ctx.accounts.reopen_account(max_data_size)
    }

    /// activates account with fee asset payment
    pub fn activate_account(ctx: Context<ActivateAccount>, user: Pubkey) -> Result<()> {
        ctx.accounts.activate_account(user)
    }

    pub fn write_data(ctx: Context<WriteData>, data: String, nonce: u64) -> Result<()> {
        ctx.accounts.write_data(data, nonce)
    }

    pub fn request_account_rotation(
        ctx: Context<RequestAccountRotation>,
        new_owner: Pubkey,
    ) -> Result<()> {
        ctx.accounts.request_account_rotation(new_owner)
    }

    /// updates address - id pair
    pub fn confirm_account_rotation(ctx: Context<ConfirmAccountRotation>) -> Result<()> {
        ctx.accounts.confirm_account_rotation()
    }
}
