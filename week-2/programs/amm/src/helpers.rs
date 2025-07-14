use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface};

use crate::state::PoolConfig;

pub fn get_space(struct_space: usize) -> usize {
    const DISCRIMINATOR_SPACE: usize = 8;

    DISCRIMINATOR_SPACE + struct_space
}

pub fn calc_sqrt(a: u64, b: u64) -> u64 {
    (a as u128 * b as u128).isqrt() as u64
}

pub fn calc_amount_out(
    amount_in: u64,
    is_mint_in_x: bool,
    total_mint_x: u64,
    total_mint_y: u64,
) -> u64 {
    let total_mint_y = total_mint_y as u128;
    let total_mint_x = total_mint_x as u128;
    let total_mint_y = total_mint_y as u128;
    let total_mint_x = total_mint_x as u128;
    let amount_in = amount_in as u128;

    let k = total_mint_x * total_mint_y;
    let amount_out = if is_mint_in_x {
        let amount_in_full = total_mint_x + amount_in;

        if amount_in_full == 0 {
            0
        } else {
            total_mint_y - k / amount_in_full
        }
    } else {
        let amount_in_full = total_mint_y + amount_in;

        if amount_in_full == 0 {
            0
        } else {
            total_mint_x - k / amount_in_full
        }
    };

    amount_out as u64
}

pub fn calc_fee(amount_out: u64, pool_config: &PoolConfig) -> u64 {
    (amount_out as u128 * pool_config.fee_bps as u128 / 10_000_u128) as u64
}

pub fn transfer_to_program<'a>(
    amount: u64,
    mint: &InterfaceAccount<'a, Mint>,
    from: &InterfaceAccount<'a, TokenAccount>,
    to: &InterfaceAccount<'a, TokenAccount>,
    signer: &Signer<'a>,
    token_program: &Interface<'a, TokenInterface>,
) -> Result<()> {
    let cpi_program = token_program.to_account_info();
    let cpi_accounts = token_interface::TransferChecked {
        from: from.to_account_info(),
        to: to.to_account_info(),
        mint: mint.to_account_info(),
        authority: signer.to_account_info(),
    };

    token_interface::transfer_checked(
        CpiContext::new(cpi_program, cpi_accounts),
        amount,
        mint.decimals,
    )
}

pub fn transfer_from_program<'a, T>(
    amount: u64,
    mint: &InterfaceAccount<'a, Mint>,
    from: &InterfaceAccount<'a, TokenAccount>,
    to: &InterfaceAccount<'a, TokenAccount>,
    seeds: &[&[u8]],
    authority: &Account<'a, T>,
    token_program: &Interface<'a, TokenInterface>,
) -> Result<()>
where
    T: AccountSerialize + AccountDeserialize + Clone,
{
    let cpi_program = token_program.to_account_info();
    let cpi_accounts = token_interface::TransferChecked {
        from: from.to_account_info(),
        to: to.to_account_info(),
        mint: mint.to_account_info(),
        authority: authority.to_account_info(),
    };

    token_interface::transfer_checked(
        CpiContext::new_with_signer(cpi_program, cpi_accounts, &[&seeds[..]]),
        amount,
        mint.decimals,
    )
}

pub fn mint_to<'a, T>(
    amount: u64,
    mint: &InterfaceAccount<'a, Mint>,
    to: &InterfaceAccount<'a, TokenAccount>,
    seeds: &[&[u8]],
    authority: &InterfaceAccount<'a, T>,
    token_program: &Interface<'a, TokenInterface>,
) -> Result<()>
where
    T: AccountSerialize + AccountDeserialize + Clone,
{
    let cpi_program = token_program.to_account_info();
    let cpi_accounts = token_interface::MintToChecked {
        mint: mint.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };

    token_interface::mint_to_checked(
        CpiContext::new_with_signer(cpi_program, cpi_accounts, &[&seeds[..]]),
        amount,
        mint.decimals,
    )
}

// x_to_withdraw = mint_x_amount * lp_shares / mint_lp_amount
// y_to_withdraw = mint_y_amount * lp_shares / mint_lp_amount
