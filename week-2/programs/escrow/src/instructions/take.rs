use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{self, Mint, TokenAccount, TokenInterface},
};

use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(id: u8)]
pub struct Take<'info> {
    // programs
    //
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    // accounts
    //
    #[account(mut)]
    maker: SystemAccount<'info>,

    #[account(mut)]
    taker: Signer<'info>,

    #[account(
        mut,
        close = maker,
        seeds = [b"escrow", maker.key.as_ref(), &[id]],
        bump
    )]
    escrow_state: Account<'info, Escrow>,

    #[account(
        mut,
        associated_token::token_program = token_program,
        associated_token::mint = maker_mint,
        associated_token::authority = escrow_state
    )]
    vault_ata_for_maker_mint: InterfaceAccount<'info, TokenAccount>,

    // probably can be moved in make
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::token_program = token_program,
        associated_token::mint = taker_mint,
        associated_token::authority = maker
    )]
    maker_ata_for_taker_mint: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::token_program = token_program,
        associated_token::mint = taker_mint,
        associated_token::authority = taker
    )]
    taker_ata_for_taker_mint: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::token_program = token_program,
        associated_token::mint = maker_mint,
        associated_token::authority = taker
    )]
    taker_ata_for_maker_mint: InterfaceAccount<'info, TokenAccount>,

    #[account()]
    maker_mint: InterfaceAccount<'info, Mint>,

    #[account()]
    taker_mint: InterfaceAccount<'info, Mint>,
}

impl<'info> Take<'info> {
    pub fn take(&mut self, id: u8, maker: Pubkey) -> Result<()> {
        let Take {
            token_program,
            maker: maker_account,
            taker,
            escrow_state,
            vault_ata_for_maker_mint,
            maker_ata_for_taker_mint,
            taker_ata_for_taker_mint,
            taker_ata_for_maker_mint,
            maker_mint,
            taker_mint,
            ..
        } = self;

        // send taker_mint tokens from taker_ata_for_taker_mint to maker_ata_for_taker_mint
        //
        let cpi_program = token_program.to_account_info();
        let cpi_accounts = token_interface::TransferChecked {
            from: taker_ata_for_taker_mint.to_account_info(),
            to: maker_ata_for_taker_mint.to_account_info(),
            mint: taker_mint.to_account_info(),
            authority: taker.to_account_info(),
        };

        token_interface::transfer_checked(
            CpiContext::new(cpi_program, cpi_accounts),
            escrow_state.taker.amount,
            taker_mint.decimals,
        )?;

        // send maker_mint tokens from vault_ata_for_maker_mint to taker_ata_for_maker_mint
        //
        let cpi_program = token_program.to_account_info();
        let cpi_accounts = token_interface::TransferChecked {
            from: vault_ata_for_maker_mint.to_account_info(),
            to: taker_ata_for_maker_mint.to_account_info(),
            mint: maker_mint.to_account_info(),
            authority: escrow_state.to_account_info(),
        };
        let seeds: &[&[u8]] = &[b"escrow", maker.as_ref(), &[id], &[escrow_state.bump]];

        token_interface::transfer_checked(
            CpiContext::new_with_signer(cpi_program, cpi_accounts, &[seeds]),
            escrow_state.maker.amount,
            maker_mint.decimals,
        )?;

        // close vault_ata_for_maker_mint
        //
        let cpi_program = token_program.to_account_info();
        let cpi_accounts = token_interface::CloseAccount {
            account: vault_ata_for_maker_mint.to_account_info(),
            destination: maker_account.to_account_info(),
            authority: escrow_state.to_account_info(),
        };

        token_interface::close_account(CpiContext::new_with_signer(
            cpi_program,
            cpi_accounts,
            &[seeds],
        ))?;

        Ok(())
    }
}
