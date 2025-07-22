use {
    crate::state::{Asset, Marketplace},
    anchor_lang::prelude::*,
    base::helpers::get_space,
};

#[derive(Accounts)]
pub struct Init<'info> {
    pub system_program: Program<'info, System>,

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
