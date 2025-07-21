use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    helpers::{get_space, mint_to},
    state::{Collection, Token},
};

#[derive(Accounts)]
#[instruction(id: u8)]
pub struct MintToken<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub recipient: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"collection", admin.key().as_ref(), id.to_le_bytes().as_ref()],
        bump = collection.bump
    )]
    pub collection: Account<'info, Collection>,

    #[account(
        init,
        payer = admin,
        space = get_space(Token::INIT_SPACE),
        seeds = [b"token", collection.address.as_ref(), collection.next_token_id.to_le_bytes().as_ref()],
        bump
    )]
    pub token: Account<'info, Token>,

    #[account(
        init,
        payer = admin,
        mint::decimals = 0,
        mint::authority = mint,
        mint::freeze_authority = mint,
        seeds = [b"mint", collection.address.as_ref(), collection.next_token_id.to_le_bytes().as_ref()],
        bump
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = admin,
        associated_token::mint = mint,
        associated_token::authority = collection
    )]
    pub app_mint_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = admin,
        associated_token::mint = mint,
        associated_token::authority = recipient
    )]
    pub recipient_mint_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> MintToken<'info> {
    pub fn mint_token(
        &mut self,
        token_bump: u8,
        mint_bump: u8,
        _id: u8,
        metadata: String,
    ) -> Result<()> {
        let MintToken {
            token_program,
            collection,
            token,
            mint,
            recipient_mint_ata,
            ..
        } = self;

        token.set_inner(Token {
            token_bump,
            mint_bump,
            id: collection.next_token_id,
            collection: collection.address,
            mint: mint.key(),
            metadata,
        });

        mint_to(
            1,
            mint,
            &recipient_mint_ata,
            &[
                b"mint",
                collection.address.as_ref(),
                collection.next_token_id.to_le_bytes().as_ref(),
            ],
            mint_bump,
            mint,
            token_program,
        )?;

        collection.next_token_id += 1;

        Ok(())
    }
}
