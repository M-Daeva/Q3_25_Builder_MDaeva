use {
    crate::state::{Asset, Marketplace},
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token_interface::{Mint, TokenAccount, TokenInterface},
    },
    base::helpers::get_space,
};

#[derive(Accounts)]
pub struct Init<'info> {
    pub system_program: Program<'info, System>,
    // pub token_program: Interface<'info, TokenInterface>,
    // pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(mut)]
    pub admin: Signer<'info>,

    // data storages
    //
    #[account(
        init,
        payer = admin,
        space = get_space(Marketplace::INIT_SPACE),
        seeds = [b"marketplace", admin.key().as_ref()],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,
    //
    // // mint
    // //
    // pub nft_mint: InterfaceAccount<'info, Mint>,

    // // ata
    // //
    // #[account(
    //     init,
    //     payer = admin,
    //     associated_token::mint = nft_mint,
    //     associated_token::authority = marketplace
    // )]
    // pub app_nft_ata: InterfaceAccount<'info, TokenAccount>,
}

impl<'info> Init<'info> {
    pub fn init(
        &mut self,
        bump: u8,
        fee_bps: u16,
        collection_whitelist: Vec<Pubkey>,
        asset_whitelist: Vec<Asset>,
        name: String,
    ) -> Result<()> {
        let Init {
            admin, marketplace, ..
        } = self;

        // TODO: guards:
        // fee_bps
        // collection_whitelist
        // asset_whitelist

        marketplace.set_inner(Marketplace {
            bump,
            admin: admin.key(),
            fee_bps,
            collection_whitelist,
            asset_whitelist,
            name,
        });

        Ok(())
    }
}
