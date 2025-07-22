use {
    crate::state::{AssetItem, Balances, Bump, Marketplace},
    anchor_lang::prelude::*,
    base::helpers::get_space,
};

#[derive(Accounts)]
pub struct Init<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        seeds = [b"treasury", admin.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>,

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
        bump: Bump,
        fee_bps: u16,
        collection_whitelist: Vec<Pubkey>,
        asset_whitelist: Vec<Pubkey>,
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

        balances.set_inner(Balances {
            value: asset_whitelist
                .iter()
                .map(|x| AssetItem {
                    amount: 0,
                    asset: x.clone(),
                })
                .collect(),
        });

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
