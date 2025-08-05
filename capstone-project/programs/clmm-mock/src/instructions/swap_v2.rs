use {
    crate::{
        error::ErrorCode,
        state::{ObservationState, PoolState},
        util::{transfer_from_pool_vault_to_user, transfer_from_user_to_pool_vault},
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        memo::Memo,
        token::Token,
        token_interface::{Mint, Token2022, TokenAccount},
    },
    raydium_clmm_cpi::states::AmmConfig,
};

#[derive(Accounts)]
pub struct SwapSingleV2<'info> {
    /// The user performing the swap
    pub payer: Signer<'info>,

    /// The factory state to read protocol fees
    #[account(address = pool_state.load()?.amm_config)]
    pub amm_config: Box<Account<'info, AmmConfig>>,

    /// The program account of the pool in which the swap will be performed
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// The user token account for input token
    #[account(mut)]
    pub input_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The user token account for output token
    #[account(mut)]
    pub output_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The vault token account for input token
    #[account(mut)]
    pub input_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The vault token account for output token
    #[account(mut)]
    pub output_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The program account for the most recent oracle observation
    #[account(mut, address = pool_state.load()?.observation_key)]
    pub observation_state: AccountLoader<'info, ObservationState>,

    /// SPL program for token transfers
    pub token_program: Program<'info, Token>,

    /// SPL program 2022 for token transfers
    pub token_program_2022: Program<'info, Token2022>,

    /// Memo program
    pub memo_program: Program<'info, Memo>,

    /// The mint of token vault 0
    #[account(
        address = input_vault.mint
    )]
    pub input_vault_mint: Box<InterfaceAccount<'info, Mint>>,

    /// The mint of token vault 1
    #[account(
        address = output_vault.mint
    )]
    pub output_vault_mint: Box<InterfaceAccount<'info, Mint>>,
}

impl<'info> SwapSingleV2<'info> {
    pub fn swap_v2(
        &mut self,
        amount: u64,
        other_amount_threshold: u64,
        _sqrt_price_limit_x64: u128,
        is_base_input: bool,
    ) -> Result<()> {
        let Self {
            payer,
            pool_state,
            input_token_account,
            output_token_account,
            input_vault,
            output_vault,
            token_program,
            token_program_2022,
            input_vault_mint,
            output_vault_mint,
            ..
        } = self;

        // Get current reserves from vaults
        let reserve_0 = input_vault.amount;
        let reserve_1 = output_vault.amount;

        // Determine if we're swapping token0 for token1 or vice versa
        let zero_for_one = input_vault.mint == pool_state.load()?.token_mint_0;

        let (amount_in, amount_out) = if is_base_input {
            // Exact input swap - calculate output using constant product formula
            let amount_in = amount;
            let amount_out = if zero_for_one {
                calculate_amount_out(amount_in, reserve_0, reserve_1)?
            } else {
                calculate_amount_out(amount_in, reserve_1, reserve_0)?
            };

            // Check slippage
            require!(
                amount_out >= other_amount_threshold,
                ErrorCode::TooLittleOutputReceived
            );

            (amount_in, amount_out)
        } else {
            // Exact output swap - calculate input using constant product formula
            let amount_out = amount;
            let amount_in = if zero_for_one {
                calculate_amount_in(amount_out, reserve_0, reserve_1)?
            } else {
                calculate_amount_in(amount_out, reserve_1, reserve_0)?
            };

            // Check slippage
            require!(
                amount_in <= other_amount_threshold,
                ErrorCode::TooMuchInputPaid
            );

            (amount_in, amount_out)
        };

        // Transfer input tokens from user to vault
        transfer_from_user_to_pool_vault(
            &payer,
            &input_token_account.to_account_info(),
            &input_vault.to_account_info(),
            Some(input_vault_mint.clone()),
            &token_program,
            Some(token_program_2022.to_account_info()),
            amount_in,
        )?;

        // Transfer output tokens from vault to user
        transfer_from_pool_vault_to_user(
            &pool_state,
            &output_vault.to_account_info(),
            &output_token_account.to_account_info(),
            Some(output_vault_mint.clone()),
            &token_program,
            Some(token_program_2022.to_account_info()),
            amount_out,
        )?;

        Ok(())
    }
}

// Helper function to calculate output amount using constant product formula
fn calculate_amount_out(amount_in: u64, reserve_in: u64, reserve_out: u64) -> Result<u64> {
    require!(amount_in > 0, ErrorCode::TooSmallInputOrOutputAmount);
    require!(
        reserve_in > 0 && reserve_out > 0,
        ErrorCode::InsufficientLiquidityForDirection
    );

    // Apply 0.3% fee (997/1000)
    let amount_in_with_fee = (amount_in as u128) * 997;
    let numerator = amount_in_with_fee * (reserve_out as u128);
    let denominator = (reserve_in as u128) * 1000 + amount_in_with_fee;

    Ok((numerator / denominator) as u64)
}

// Helper function to calculate input amount using constant product formula
fn calculate_amount_in(amount_out: u64, reserve_in: u64, reserve_out: u64) -> Result<u64> {
    require!(amount_out > 0, ErrorCode::TooSmallInputOrOutputAmount);
    require!(
        reserve_in > 0 && reserve_out > 0,
        ErrorCode::InsufficientLiquidityForDirection
    );
    require!(
        amount_out < reserve_out,
        ErrorCode::InsufficientLiquidityForDirection
    );

    let numerator = (reserve_in as u128) * (amount_out as u128) * 1000;
    let denominator = (reserve_out as u128 - amount_out as u128) * 997;

    Ok((numerator / denominator + 1) as u64) // Add 1 to round up
}
