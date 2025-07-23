use {
    crate::state::{Marketplace, Trade},
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token_interface::{Mint, TokenAccount, TokenInterface},
    },
    base::helpers::{transfer_sol_from_program, transfer_token_from_program},
};

#[derive(Accounts)]
#[instruction(collection: Pubkey, token_id: u16)]
pub struct RemoveBuyWithTokenTrade<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    pub admin: SystemAccount<'info>,

    // data storage
    //
    #[account(
        seeds = [b"marketplace", admin.key().as_ref()],
        bump = marketplace.bump.marketplace
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        mut,
        close = buyer,
        seeds = [b"trade", buyer.key().as_ref(), collection.as_ref(), token_id.to_le_bytes().as_ref()],
        bump = trade.bump
    )]
    pub trade: Account<'info, Trade>,

    // mint
    //
    pub token_mint: InterfaceAccount<'info, Mint>,

    // ata
    //
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = buyer
    )]
    pub buyer_token_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = marketplace
    )]
    pub app_token_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> RemoveBuyWithTokenTrade<'info> {
    pub fn remove_buy_with_token_trade(
        &mut self,
        _collection: Pubkey,
        _token_id: u16,
    ) -> Result<()> {
        let RemoveBuyWithTokenTrade {
            token_program,
            admin,
            marketplace,
            trade,
            token_mint,
            buyer_token_ata,
            app_token_ata,
            ..
        } = self;

        transfer_token_from_program(
            trade.price.amount,
            token_mint,
            app_token_ata,
            buyer_token_ata,
            &[b"marketplace", admin.key().as_ref()],
            marketplace.bump.marketplace,
            marketplace,
            token_program,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(collection: Pubkey, token_id: u16)]
pub struct RemoveBuyWithSolTrade<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    pub admin: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"treasury", admin.key().as_ref()],
        bump = marketplace.bump.treasury
    )]
    pub treasury: SystemAccount<'info>,

    // data storage
    //
    #[account(
        seeds = [b"marketplace", admin.key().as_ref()],
        bump = marketplace.bump.marketplace
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        mut,
        close = buyer,
        seeds = [b"trade", buyer.key().as_ref(), collection.as_ref(), token_id.to_le_bytes().as_ref()],
        bump = trade.bump
    )]
    pub trade: Account<'info, Trade>,
}

impl<'info> RemoveBuyWithSolTrade<'info> {
    pub fn remove_buy_with_sol_trade(&mut self, _collection: Pubkey, _token_id: u16) -> Result<()> {
        let RemoveBuyWithSolTrade {
            system_program,
            buyer,
            admin,
            treasury,
            marketplace,
            trade,
        } = self;

        transfer_sol_from_program(
            trade.price.amount,
            &treasury.to_account_info(),
            buyer,
            &[b"treasury", admin.key().as_ref()],
            marketplace.bump.treasury,
            system_program,
        )?;

        Ok(())
    }
}
