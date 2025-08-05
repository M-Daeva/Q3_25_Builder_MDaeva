#![allow(unexpected_cfgs)]
#![allow(deprecated)]

pub mod error;
pub mod instructions;
pub mod state;

use {
    anchor_lang::prelude::*,
    instructions::{create_operation_account::*, create_pool::*, swap_v2::*},
};

declare_id!("AyzvQE5M1Xqs4YQxP4Sf6giH82X4bKFUF3oVv7DTkoD4");

#[program]
pub mod clmm_mock {
    use super::*;

    pub fn create_operation_account(ctx: Context<CreateOperationAccount>) -> Result<()> {
        instructions::create_operation_account(ctx)
    }

    pub fn create_pool(ctx: Context<CreatePool>, amount_a: u64, amount_b: u64) -> Result<()> {
        ctx.accounts.create_pool(ctx.bumps, amount_a, amount_b)
    }

    pub fn swap_v2(
        ctx: Context<SwapSingleV2>,
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit_x64: u128,
        is_base_input: bool,
    ) -> Result<()> {
        ctx.accounts.swap_v2(
            amount,
            other_amount_threshold,
            sqrt_price_limit_x64,
            is_base_input,
        )
    }
}
