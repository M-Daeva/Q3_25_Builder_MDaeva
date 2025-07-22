use {
    crate::state::{Asset, Balances, Marketplace},
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

    #[account(
        init,
        payer = admin,
        space = get_space(Balances::INIT_SPACE),
        seeds = [b"balances", admin.key().as_ref()],
        bump
    )]
    pub balances: Account<'info, Balances>,
}

impl<'info> Init<'info> {
    pub fn init(
        &mut self,
        marketplace_bump: u8,
        balances_bump: u8,
        fee_bps: u16,
        collection_whitelist: Vec<Pubkey>,
        asset_whitelist: Vec<Asset>,
        name: String,
    ) -> Result<()> {
        let Init {
            admin,
            marketplace,
            balances,
            ..
        } = self;

        // TODO: guards:
        // fee_bps
        // collection_whitelist
        // asset_whitelist

        marketplace.set_inner(Marketplace {
            marketplace_bump,
            balances_bump,
            admin: admin.key(),
            fee_bps,
            collection_whitelist,
            asset_whitelist,
            name,
        });

        balances.set_inner(Balances { value: vec![] });

        Ok(())
    }
}
