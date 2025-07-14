use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    error::ProgError,
    helpers::{calc_amount_out, calc_fee, transfer_from_program, transfer_to_program},
    state::{PoolBalance, PoolConfig},
};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct Swap<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(mut)]
    pub trader: Signer<'info>,

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

    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = trader,
        associated_token::token_program = token_program,
        associated_token::mint = mint_x,
        associated_token::authority = trader,
    )]
    pub trader_mint_x_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = trader,
        associated_token::token_program = token_program,
        associated_token::mint = mint_y,
        associated_token::authority = trader,
    )]
    pub trader_mint_y_ata: Box<InterfaceAccount<'info, TokenAccount>>,

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

impl<'info> Swap<'info> {
    pub fn swap(&mut self, amount_in: u64, mint_in: Pubkey) -> Result<()> {
        let Swap {
            token_program,
            trader,
            pool_config,
            pool_balance,
            mint_x,
            mint_y,
            trader_mint_x_ata,
            trader_mint_y_ata,
            liquidity_pool_mint_x_ata,
            liquidity_pool_mint_y_ata,
            ..
        } = self;

        if mint_in != pool_config.mint_x && mint_in != pool_config.mint_y {
            Err(ProgError::WrongMint)?;
        }

        let is_mint_in_x = mint_in == pool_config.mint_x;
        let amount_out = calc_amount_out(
            amount_in,
            is_mint_in_x,
            pool_balance.mint_x_amount,
            pool_balance.mint_y_amount,
        );

        let fee = calc_fee(amount_out, pool_config);
        let amount_to_send = amount_out - fee;

        if fee == 0 {
            Err(ProgError::NoLiquidity)?;
        }

        if is_mint_in_x {
            pool_balance.mint_x_amount += amount_in;
            pool_balance.mint_y_amount -= amount_to_send;
        } else {
            pool_balance.mint_y_amount += amount_in;
            pool_balance.mint_x_amount -= amount_to_send;
        }

        // send tokens from the trader to the pool
        let (mint, from, to) = if is_mint_in_x {
            (&mint_x, &trader_mint_x_ata, &liquidity_pool_mint_x_ata)
        } else {
            (&mint_y, &trader_mint_y_ata, &liquidity_pool_mint_y_ata)
        };
        transfer_to_program(amount_in, mint, from, to, trader, token_program)?;

        // send tokens from program to trader
        let (mint, from, to) = if is_mint_in_x {
            (&mint_y, &liquidity_pool_mint_y_ata, &trader_mint_y_ata)
        } else {
            (&mint_x, &liquidity_pool_mint_x_ata, &trader_mint_x_ata)
        };
        transfer_from_program(
            amount_to_send,
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

        Ok(())
    }
}
