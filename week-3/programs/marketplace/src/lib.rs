#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

use {
    instructions::{
        accept_buy_trade::*, accept_sell_trade::*, create_buy_trade::*, create_sell_trade::*,
        init::*, remove_buy_trade::*, remove_sell_trade::*, withdraw_fee::*,
    },
    state::*,
};

declare_id!("DWTez7iNT13kaqEEZfzFQb5Msv7JdF9EFKRize99wQTi");

#[program]
pub mod marketplace {
    use super::*;

    pub fn init(
        ctx: Context<Init>,
        fee_bps: u16,
        collection_whitelist: Vec<Pubkey>,
        asset_whitelist: Vec<Pubkey>,
        name: String,
    ) -> Result<()> {
        ctx.accounts.init(
            Bump {
                marketplace: ctx.bumps.marketplace,
                balances: ctx.bumps.balances,
                treasury: ctx.bumps.treasury,
            },
            fee_bps,
            collection_whitelist,
            asset_whitelist,
            name,
        )
    }

    pub fn create_sell_for_token_trade(
        ctx: Context<CreateSellForTokenTrade>,
        collection: Pubkey,
        token_id: u16,
        price: AssetItem,
    ) -> Result<()> {
        ctx.accounts
            .create_sell_for_token_trade(ctx.bumps.trade, collection, token_id, price)
    }

    pub fn create_sell_for_sol_trade(
        ctx: Context<CreateSellForSolTrade>,
        collection: Pubkey,
        token_id: u16,
        price: AssetItem,
    ) -> Result<()> {
        ctx.accounts
            .create_sell_for_sol_trade(ctx.bumps.trade, collection, token_id, price)
    }

    pub fn create_buy_with_token_trade(
        ctx: Context<CreateBuyWithTokenTrade>,
        collection: Pubkey,
        token_id: u16,
        price: AssetItem,
    ) -> Result<()> {
        ctx.accounts
            .create_buy_with_token_trade(ctx.bumps.trade, collection, token_id, price)
    }

    pub fn create_buy_with_sol_trade(
        ctx: Context<CreateBuyWithSolTrade>,
        collection: Pubkey,
        token_id: u16,
        price: AssetItem,
    ) -> Result<()> {
        ctx.accounts
            .create_buy_with_sol_trade(ctx.bumps.trade, collection, token_id, price)
    }

    pub fn accept_sell_for_token_trade(
        ctx: Context<AcceptSellForTokenTrade>,
        collection: Pubkey,
        token_id: u16,
    ) -> Result<()> {
        ctx.accounts
            .accept_sell_for_token_trade(collection, token_id)
    }

    pub fn accept_sell_for_sol_trade(
        ctx: Context<AcceptSellForSolTrade>,
        collection: Pubkey,
        token_id: u16,
    ) -> Result<()> {
        ctx.accounts.accept_sell_for_sol_trade(collection, token_id)
    }

    pub fn accept_buy_with_token_trade(
        ctx: Context<AcceptBuyWithTokenTrade>,
        collection: Pubkey,
        token_id: u16,
    ) -> Result<()> {
        ctx.accounts
            .accept_buy_with_token_trade(collection, token_id)
    }

    pub fn accept_buy_with_sol_trade(
        ctx: Context<AcceptBuyWithSolTrade>,
        collection: Pubkey,
        token_id: u16,
    ) -> Result<()> {
        ctx.accounts.accept_buy_with_sol_trade(collection, token_id)
    }

    pub fn withdraw_token_fee(ctx: Context<WithdrawTokenFee>) -> Result<()> {
        ctx.accounts.withdraw_token_fee()
    }

    pub fn withdraw_sol_fee(ctx: Context<WithdrawSolFee>) -> Result<()> {
        ctx.accounts.withdraw_sol_fee()
    }

    pub fn remove_sell_trade(
        ctx: Context<RemoveSellTrade>,
        collection: Pubkey,
        token_id: u16,
    ) -> Result<()> {
        ctx.accounts.remove_sell_trade(collection, token_id)
    }

    pub fn remove_buy_with_token_trade(
        ctx: Context<RemoveBuyWithTokenTrade>,
        collection: Pubkey,
        token_id: u16,
    ) -> Result<()> {
        ctx.accounts
            .remove_buy_with_token_trade(collection, token_id)
    }

    pub fn remove_buy_with_sol_trade(
        ctx: Context<RemoveBuyWithSolTrade>,
        collection: Pubkey,
        token_id: u16,
    ) -> Result<()> {
        ctx.accounts.remove_buy_with_sol_trade(collection, token_id)
    }
}
