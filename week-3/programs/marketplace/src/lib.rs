#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

use {
    instructions::{
        accept_buy_trade::*, accept_sell_trade::*, create_buy_trade::*, create_sell_trade::*,
        init::*, withdraw_fee::*,
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
        asset_whitelist: Vec<Asset>,
        name: String,
    ) -> Result<()> {
        ctx.accounts.init(
            ctx.bumps.marketplace,
            ctx.bumps.balances,
            fee_bps,
            collection_whitelist,
            asset_whitelist,
            name,
        )
    }

    pub fn create_sell_trade(
        ctx: Context<CreateSellTrade>,
        collection: Pubkey,
        token_id: u16,
        price_amount: u64,
        price_asset: Asset,
    ) -> Result<()> {
        ctx.accounts.create_sell_trade(
            ctx.bumps.trade,
            collection,
            token_id,
            price_amount,
            price_asset,
        )
    }

    pub fn create_buy_trade(
        ctx: Context<CreateBuyTrade>,
        collection: Pubkey,
        token_id: u16,
        price_amount: u64,
        price_asset: Asset,
    ) -> Result<()> {
        ctx.accounts.create_buy_trade(
            ctx.bumps.trade,
            collection,
            token_id,
            price_amount,
            price_asset,
        )
    }

    pub fn accept_sell_trade(
        ctx: Context<AcceptSellTrade>,
        collection: Pubkey,
        token_id: u16,
    ) -> Result<()> {
        ctx.accounts.accept_sell_trade(collection, token_id)
    }

    pub fn accept_buy_trade(
        ctx: Context<AcceptBuyTrade>,
        collection: Pubkey,
        token_id: u16,
    ) -> Result<()> {
        ctx.accounts.accept_buy_trade(collection, token_id)
    }

    pub fn withdraw_fee(ctx: Context<WithdrawFee>) -> Result<()> {
        ctx.accounts.withdraw_fee()
    }
}

// TODO: remove_trade
