#![allow(unexpected_cfgs)]
#![allow(deprecated)]

pub mod error;
pub mod instructions;
pub mod state;
pub mod util;

use {
    anchor_lang::prelude::*,
    instructions::{
        create_amm_config::*, create_operation_account::*, create_pool::*,
        open_position_with_token22_nft::*, swap_v2::*,
    },
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

    pub fn create_pool(
        ctx: Context<CreatePool>,
        sqrt_price_x64: u128,
        open_time: u64,
    ) -> Result<()> {
        instructions::create_pool(ctx, sqrt_price_x64, open_time)
    }

    pub fn open_position_with_token22_nft<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, OpenPositionWithToken22Nft<'info>>,
        tick_lower_index: i32,
        tick_upper_index: i32,
        tick_array_lower_start_index: i32,
        tick_array_upper_start_index: i32,
        liquidity: u128,
        amount_0_max: u64,
        amount_1_max: u64,
        with_metadata: bool,
        base_flag: Option<bool>,
    ) -> Result<()> {
        instructions::open_position_with_token22_nft(
            ctx,
            liquidity,
            amount_0_max,
            amount_1_max,
            tick_lower_index,
            tick_upper_index,
            tick_array_lower_start_index,
            tick_array_upper_start_index,
            with_metadata,
            base_flag,
        )
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
