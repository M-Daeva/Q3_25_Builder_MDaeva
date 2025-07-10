use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{self, Mint, TokenAccount, TokenInterface},
};

use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(id: u8)]
pub struct Refund<'info> {
    // programs
    //
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    //

    // accounts
    //
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut,
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), &[id]],
        bump = escrow_state.bump
    )]
    pub escrow_state: Account<'info, Escrow>,

    #[account(
        mut,
        associated_token::token_program = token_program,
        associated_token::mint = maker_mint,
        associated_token::authority = escrow_state
    )]
    pub vault_ata_for_maker_mint: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::token_program = token_program,
        associated_token::mint = maker_mint,
        associated_token::authority = maker
    )]
    pub maker_ata_for_maker_mint: InterfaceAccount<'info, TokenAccount>,

    #[account()]
    pub maker_mint: InterfaceAccount<'info, Mint>,
}

impl<'info> Refund<'info> {
    pub fn refund(&mut self, id: u8) -> Result<()> {
        let Refund {
            token_program,
            maker,
            escrow_state,
            vault_ata_for_maker_mint,
            maker_ata_for_maker_mint,
            maker_mint,
            ..
        } = self;

        // return maker_mint tokens to maker_ata_for_maker_mint
        //
        let cpi_program = token_program.to_account_info();
        let cpi_accounts = token_interface::TransferChecked {
            from: vault_ata_for_maker_mint.to_account_info(),
            to: maker_ata_for_maker_mint.to_account_info(),
            mint: maker_mint.to_account_info(),
            authority: escrow_state.to_account_info(),
        };
        let seeds: &[&[u8]] = &[b"escrow", maker.key.as_ref(), &[id], &[escrow_state.bump]];

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
            destination: maker.to_account_info(),
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
