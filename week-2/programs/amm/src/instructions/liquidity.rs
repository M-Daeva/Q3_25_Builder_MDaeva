use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    error::ProgError,
    helpers::{burn_from, mint_to, transfer_from_program, transfer_to_program},
    math::{calc_shares, calc_sqrt},
    state::{PoolBalance, PoolConfig},
};

// Box was used to fix
// Error: Function _ZN163_$LT$amm..instructions..provide_liquidity..ProvideLiquidity$u20$as$u20$anchor_lang..Accounts$LT$amm..instructions..provide_liquidity..
// ProvideLiquidityBumps$GT$$GT$12try_accounts17h32b29a029ae8914fE Stack offset of 4296 exceeded max offset of 4096 by 200 bytes, please minimize large stack variables.
// Estimated function frame size: 4800 bytes. Exceeding the maximum stack offset may cause undefined behavior during execution.
#[derive(Accounts)]
#[instruction(id: u64)]
pub struct Liquidity<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(mut)]
    pub liquidity_provider: Signer<'info>,

    #[account(
        seeds = [b"config", id.to_le_bytes().as_ref()],
        bump = pool_config.config_bump
    )]
    pub pool_config: Box<Account<'info, PoolConfig>>,

    #[account(
        mut,
        seeds = [b"balance", id.to_le_bytes().as_ref()],
        bump = pool_config.balance_bump
    )]
    pub pool_balance: Box<Account<'info, PoolBalance>>,

    #[account(
        mut,
        seeds = [b"lp", id.to_le_bytes().as_ref()],
        bump = pool_config.lp_bump
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = liquidity_provider,
        associated_token::token_program = token_program,
        associated_token::mint = mint_lp,
        associated_token::authority = liquidity_provider,
    )]
    pub liquidity_provider_mint_lp_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = liquidity_provider,
        associated_token::token_program = token_program,
        associated_token::mint = mint_x,
        associated_token::authority = liquidity_provider,
    )]
    pub liquidity_provider_mint_x_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = liquidity_provider,
        associated_token::token_program = token_program,
        associated_token::mint = mint_y,
        associated_token::authority = liquidity_provider,
    )]
    pub liquidity_provider_mint_y_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::token_program = token_program,
        associated_token::mint = mint_lp,
        associated_token::authority = pool_config,
    )]
    pub liquidity_pool_mint_lp_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::token_program = token_program,
        associated_token::mint = mint_x,
        associated_token::authority = pool_config,
    )]
    pub liquidity_pool_mint_x_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::token_program = token_program,
        associated_token::mint = mint_y,
        associated_token::authority = pool_config,
    )]
    pub liquidity_pool_mint_y_ata: Box<InterfaceAccount<'info, TokenAccount>>,
}

impl<'info> Liquidity<'info> {
    pub fn provide_liquidity(&mut self, mint_x_amount: u64, mint_y_amount: u64) -> Result<()> {
        let Liquidity {
            token_program,
            liquidity_provider,
            pool_config,
            pool_balance,
            mint_lp,
            mint_x,
            mint_y,
            liquidity_provider_mint_lp_ata,
            liquidity_provider_mint_x_ata,
            liquidity_provider_mint_y_ata,
            liquidity_pool_mint_x_ata,
            liquidity_pool_mint_y_ata,
            ..
        } = self;

        // it's only possible to provide two-sided liquidity
        if mint_x_amount == 0 || mint_y_amount == 0 {
            Err(ProgError::NoLiquidity)?;
        }

        let mint_lp_amount = calc_sqrt(mint_x_amount, mint_y_amount);

        pool_balance.mint_x_amount += mint_x_amount;
        pool_balance.mint_y_amount += mint_y_amount;
        pool_balance.mint_lp_amount += mint_lp_amount;

        // send tokens from the liquidity provider to the pool
        for (amount, mint, from, to) in [
            (
                mint_x_amount,
                mint_x,
                liquidity_provider_mint_x_ata,
                liquidity_pool_mint_x_ata,
            ),
            (
                mint_y_amount,
                mint_y,
                liquidity_provider_mint_y_ata,
                liquidity_pool_mint_y_ata,
            ),
        ] {
            transfer_to_program(amount, mint, from, to, liquidity_provider, token_program)?;
        }

        // mint tokens to the liquidity provider
        mint_to(
            mint_lp_amount,
            mint_lp,
            liquidity_provider_mint_lp_ata,
            &[
                b"lp",
                pool_config.id.to_le_bytes().as_ref(),
                &[pool_config.lp_bump],
            ],
            mint_lp,
            token_program,
        )?;

        Ok(())
    }
}

impl<'info> Liquidity<'info> {
    pub fn withdraw_liquidity(&mut self, mint_lp_amount: u64) -> Result<()> {
        let Liquidity {
            token_program,
            liquidity_provider,
            pool_config,
            pool_balance,
            mint_lp,
            mint_x,
            mint_y,
            liquidity_provider_mint_lp_ata,
            liquidity_provider_mint_x_ata,
            liquidity_provider_mint_y_ata,
            liquidity_pool_mint_x_ata,
            liquidity_pool_mint_y_ata,
            ..
        } = self;

        let mint_x_amount = calc_shares(
            mint_lp_amount,
            true,
            pool_balance.mint_x_amount,
            pool_balance.mint_y_amount,
            pool_balance.mint_lp_amount,
        );
        let mint_y_amount = calc_shares(
            mint_lp_amount,
            false,
            pool_balance.mint_x_amount,
            pool_balance.mint_y_amount,
            pool_balance.mint_lp_amount,
        );

        if mint_x_amount == 0 || mint_y_amount == 0 {
            Err(ProgError::NoLiquidity)?;
        }

        pool_balance.mint_x_amount -= mint_x_amount;
        pool_balance.mint_y_amount -= mint_y_amount;
        pool_balance.mint_lp_amount -= mint_lp_amount;

        // burn lp tokens
        burn_from(
            mint_lp_amount,
            mint_lp,
            liquidity_provider_mint_lp_ata,
            liquidity_provider,
            token_program,
        )?;

        // send tokens from the pool to the liquidity provider
        for (amount, mint, from, to) in [
            (
                mint_x_amount,
                mint_x,
                liquidity_pool_mint_x_ata,
                liquidity_provider_mint_x_ata,
            ),
            (
                mint_y_amount,
                mint_y,
                liquidity_pool_mint_y_ata,
                liquidity_provider_mint_y_ata,
            ),
        ] {
            transfer_from_program(
                amount,
                mint,
                from,
                to,
                &[
                    b"config",
                    pool_config.id.to_le_bytes().as_ref(),
                    &[pool_config.config_bump],
                ],
                pool_config,
                token_program,
            )?;
        }

        Ok(())
    }
}
