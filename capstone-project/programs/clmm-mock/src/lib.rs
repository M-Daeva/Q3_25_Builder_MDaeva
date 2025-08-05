#![allow(unexpected_cfgs)]
#![allow(deprecated)]

pub mod error;
pub mod instructions;
pub mod state;

use {
    anchor_lang::prelude::*,
    instructions::{create_amm_config::*, create_operation_account::*, create_pool::*, swap_v2::*},
    raydium_clmm_cpi::states::FEE_RATE_DENOMINATOR_VALUE,
};

// same as original program id
declare_id!("CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK");

#[program]
pub mod clmm_mock {
    use super::*;

    pub fn create_operation_account(ctx: Context<CreateOperationAccount>) -> Result<()> {
        instructions::create_operation_account(ctx)
    }

    pub fn create_amm_config(
        ctx: Context<CreateAmmConfig>,
        index: u16,
        tick_spacing: u16,
        trade_fee_rate: u32,
        protocol_fee_rate: u32,
        fund_fee_rate: u32,
    ) -> Result<()> {
        assert!(trade_fee_rate < FEE_RATE_DENOMINATOR_VALUE);
        assert!(protocol_fee_rate <= FEE_RATE_DENOMINATOR_VALUE);
        assert!(fund_fee_rate <= FEE_RATE_DENOMINATOR_VALUE);
        assert!(fund_fee_rate + protocol_fee_rate <= FEE_RATE_DENOMINATOR_VALUE);
        instructions::create_amm_config(
            ctx,
            index,
            tick_spacing,
            trade_fee_rate,
            protocol_fee_rate,
            fund_fee_rate,
        )
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
