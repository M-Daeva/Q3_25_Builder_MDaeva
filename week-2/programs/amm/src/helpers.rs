use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface};

pub fn get_space(struct_space: usize) -> usize {
    const DISCRIMINATOR_SPACE: usize = 8;

    DISCRIMINATOR_SPACE + struct_space
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

pub fn burn_from<'a>(
    amount: u64,
    mint: &InterfaceAccount<'a, Mint>,
    from: &InterfaceAccount<'a, TokenAccount>,
    signer: &Signer<'a>,
    token_program: &Interface<'a, TokenInterface>,
) -> Result<()> {
    let cpi_program = token_program.to_account_info();
    let cpi_accounts = token_interface::BurnChecked {
        mint: mint.to_account_info(),
        from: from.to_account_info(),
        authority: signer.to_account_info(),
    };

    token_interface::burn_checked(
        CpiContext::new(cpi_program, cpi_accounts),
        amount,
        mint.decimals,
    )
}
