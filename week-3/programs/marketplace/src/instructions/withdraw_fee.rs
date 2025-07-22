use {
    crate::{
        error::CustomError,
        state::{Asset, Marketplace, Trade},
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token_interface::{Mint, TokenAccount, TokenInterface},
    },
    base::{
        error::NftError,
        helpers::{deserialize_account, get_space, transfer_from_program, transfer_from_user},
    },
};

#[derive(Accounts)]
pub struct WithdrawFee<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(mut)]
    pub sender: Signer<'info>,

    pub admin: SystemAccount<'info>,

    // data storage
    //
    #[account(
        seeds = [b"marketplace", admin.key().as_ref()],
        bump = marketplace.marketplace_bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    // mint
    //
    pub token_mint: InterfaceAccount<'info, Mint>,

    // ata
    //
    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = token_mint,
        associated_token::authority = sender
    )]
    pub sender_token_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = marketplace
    )]
    pub app_token_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> WithdrawFee<'info> {
    pub fn withdraw_fee(&mut self) -> Result<()> {
        Ok(())
    }
}
