#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

pub mod helpers;
pub mod instructions;

use {
    dex_adapter_cpi::types::RouteItem,
    instructions::{
        init::*, save_route::*, swap_and_activate::*, swap_and_unwrap_wsol::*, swap_multihop::*,
    },
};

// IDL builder doesn't see ID from cpi package, we need to duplicate it here
declare_id!("FMsjKKPk7FQb1B9H8UQTLrdCUZ9MaoAeTnNK9kdVJmtt");

#[program]
pub mod dex_adapter {
    use super::*;

    pub fn init(
        ctx: Context<Init>,
        dex: Pubkey,
        registry: Option<Pubkey>,
        rotation_timeout: Option<u32>,
        token_in_whitelist: Option<Vec<Pubkey>>,
    ) -> Result<()> {
        ctx.accounts.init(
            ctx.bumps,
            dex,
            registry,
            rotation_timeout,
            token_in_whitelist,
        )
    }

    // pub fn update_config(
    //     admin: Option<Pubkey>,
    //     dex: Option<Pubkey>,
    //     registry: Option<Pubkey>,
    //     is_paused: Option<bool>,
    //     rotation_timeout: Option<u32>,
    //     token_in_whitelist: Option<Vec<Pubkey>>,
    // ) -> Result<()> {
    //     unimplemented!()
    // }

    // pub fn confirm_admin_rotation() -> Result<()> {
    //     unimplemented!()
    // }

    pub fn save_route(
        ctx: Context<SaveRoute>,
        mint_first: Pubkey,
        mint_last: Pubkey,
        route: Vec<RouteItem>,
    ) -> Result<()> {
        ctx.accounts.save_route(mint_first, mint_last, route)
    }

    /// swap across multiple pools
    pub fn swap_multihop<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, SwapMultihop<'info>>,
        amount_in: u64,
        amount_out_minimum: u64,
    ) -> Result<()> {
        ctx.accounts
            .swap_multihop(ctx.remaining_accounts, amount_in, amount_out_minimum)
    }

    /// swap tokens and call activate_account of registry program
    pub fn swap_and_activate<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, SwapAndActivate<'info>>,
        amount_in: u64,
        amount_out_minimum: u64,
    ) -> Result<()> {
        ctx.accounts
            .swap_and_activate(ctx.remaining_accounts, amount_in, amount_out_minimum)
    }

    /// swap a token to WSOL and unwrap it to SOL
    pub fn swap_and_unwrap_wsol<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, SwapAndUnwrapWsol<'info>>,
        amount_in: u64,
        amount_out_minimum: u64,
    ) -> Result<()> {
        ctx.accounts
            .swap_and_unwrap_wsol(ctx.remaining_accounts, amount_in, amount_out_minimum)
    }
}
