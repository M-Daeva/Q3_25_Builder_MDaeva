#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

pub mod error;
pub mod helpers;
pub mod instructions;
pub mod math;
pub mod state;

use instructions::{create_pool::*, liquidity::*, swap::*};

declare_id!("CpuYGzAZWKWBHXUoBSfEg3qnvRd8pMcRa9XV29Xoj3KU");

// to avoid "multiple definition of `entrypoint'"" error
// also, use in tests Cargo.toml: <program> = { workspace = true, features = ["cpi"] }
#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

#[program]
pub mod amm {
    use super::*;

    pub fn create_pool(
        ctx: Context<CreatePool>,
        id: u64,
        mint_x: Pubkey,
        mint_y: Pubkey,
        fee_bps: u16,
    ) -> Result<()> {
        ctx.accounts.create_pool(
            id,
            ctx.bumps.pool_config,
            ctx.bumps.pool_balance,
            ctx.bumps.mint_lp,
            mint_x,
            mint_y,
            fee_bps,
        )
    }

    pub fn provide_liquidity(
        ctx: Context<Liquidity>,
        _id: u64,
        mint_x_amount: u64,
        mint_y_amount: u64,
    ) -> Result<()> {
        ctx.accounts.provide_liquidity(mint_x_amount, mint_y_amount)
    }

    pub fn withdraw_liquidity(
        ctx: Context<Liquidity>,
        _id: u64,
        mint_lp_amount: u64,
    ) -> Result<()> {
        ctx.accounts.withdraw_liquidity(mint_lp_amount)
    }

    pub fn swap(ctx: Context<Swap>, _id: u64, amount_in: u64, mint_in: Pubkey) -> Result<()> {
        ctx.accounts.swap(amount_in, mint_in)
    }
}
