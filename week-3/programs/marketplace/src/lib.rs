#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

use {
    instructions::{create_trade::*, init::*},
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
            fee_bps,
            collection_whitelist,
            asset_whitelist,
            name,
        )
    }

    pub fn create_trade(
        ctx: Context<CreateTrade>,
        is_sell_nft_trade: bool,
        collection: Pubkey,
        token_id: u16,
        price_amount: u64,
        price_asset: Asset,
    ) -> Result<()> {
        ctx.accounts.create_trade(
            ctx.bumps.trade,
            is_sell_nft_trade,
            collection,
            token_id,
            price_amount,
            price_asset,
        )
    }
}

// TODO: remove_trade
// TODO: accept_trade
// TODO: withdraw_fee
