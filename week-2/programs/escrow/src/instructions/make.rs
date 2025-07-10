use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{self, Mint, TokenAccount, TokenInterface},
};

use crate::{
    helpers::get_space,
    state::{Escrow, TraderInfo},
};

#[derive(Accounts)]
#[instruction(id: u8)] // reference to entry point, not to the method
pub struct Make<'info> {
    // programs
    //
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    // accounts
    //
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        init_if_needed,
        space = get_space(Escrow::INIT_SPACE),
        payer = maker,
        seeds = [b"escrow", maker.key().as_ref(), &[id]],
        bump
    )]
    pub escrow_state: Account<'info, Escrow>,

    #[account(
        init_if_needed,
        payer = maker,
        associated_token::token_program = token_program,
        associated_token::mint = maker_mint,
        associated_token::authority = escrow_state
    )]
    pub vault_ata_for_maker_mint: InterfaceAccount<'info, TokenAccount>,

    // if maker gonna deposit tokens then he already has ata
    #[account(
        mut,
        associated_token::token_program = token_program,
        associated_token::mint = maker_mint,
        associated_token::authority = maker
    )]
    pub maker_ata_for_maker_mint: InterfaceAccount<'info, TokenAccount>,

    // required just to pass in ata
    #[account()]
    pub maker_mint: InterfaceAccount<'info, Mint>,
}

impl<'info> Make<'info> {
    pub fn make(&mut self, bump: u8, id: u8, maker: TraderInfo, taker: TraderInfo) -> Result<()> {
        let amount_to_send = maker.amount;
        let Make {
            token_program,
            maker: maker_signer,
            escrow_state,
            vault_ata_for_maker_mint,
            maker_ata_for_maker_mint,
            maker_mint,
            ..
        } = self;

        escrow_state.set_inner(Escrow {
            bump,
            id,
            maker,
            taker,
        });

        let cpi_program = token_program.to_account_info();
        let cpi_accounts = token_interface::TransferChecked {
            from: maker_ata_for_maker_mint.to_account_info(),
            mint: maker_mint.to_account_info(),
            to: vault_ata_for_maker_mint.to_account_info(),
            authority: maker_signer.to_account_info(),
        };

        token_interface::transfer_checked(
            CpiContext::new(cpi_program, cpi_accounts),
            amount_to_send,
            maker_mint.decimals,
        )
    }
}
