use {
    crate::state::{Marketplace, Trade},
    anchor_lang::prelude::*,
    anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface},
    base::helpers::transfer_token_from_program,
};

#[derive(Accounts)]
#[instruction(collection: Pubkey, token_id: u16)]
pub struct RemoveSellTrade<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    /// CHECK: nft_program
    pub nft_program: AccountInfo<'info>,

    #[account(mut)]
    pub seller: Signer<'info>,

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
        close = seller,
        seeds = [b"trade", seller.key().as_ref(), collection.as_ref(), token_id.to_le_bytes().as_ref()],
        bump = trade.bump
    )]
    pub trade: Account<'info, Trade>,

    // mint
    //
    pub nft_mint: InterfaceAccount<'info, Mint>,

    // ata
    //
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = seller
    )]
    pub seller_nft_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = marketplace
    )]
    pub app_nft_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> RemoveSellTrade<'info> {
    pub fn remove_sell_trade(&mut self, _collection: Pubkey, _token_id: u16) -> Result<()> {
        let RemoveSellTrade {
            token_program,
            admin,
            marketplace,
            nft_mint,
            seller_nft_ata,
            app_nft_ata,
            ..
        } = self;

        transfer_token_from_program(
            1,
            nft_mint,
            app_nft_ata,
            seller_nft_ata,
            &[b"marketplace", admin.key().as_ref()],
            marketplace.bump.marketplace,
            marketplace,
            token_program,
        )?;

        Ok(())
    }
}
