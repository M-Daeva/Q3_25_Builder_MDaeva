use {
    crate::{
        error::CustomError,
        state::{AssetItem, Balances, Bump, Marketplace},
    },
    anchor_lang::prelude::*,
    base::{
        error::NftError,
        helpers::{get_rent_exempt, get_space, has_duplicates, transfer_sol_from_user},
    },
};

#[derive(Accounts)]
pub struct Init<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
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
            system_program,
            admin,
            treasury,
            marketplace,
            balances,
        } = self;

        if fee_bps > 10_000 {
            Err(CustomError::FeeIsTooBig)?;
        }

        if collection_whitelist.is_empty() {
            Err(NftError::EmptyCollectionList)?;
        }

        if has_duplicates(&collection_whitelist) {
            Err(NftError::CollectionDuplication)?;
        }

        if asset_whitelist.is_empty() {
            Err(CustomError::EmptyAssetList)?;
        }

        if has_duplicates(&asset_whitelist) {
            Err(CustomError::AssetDuplication)?;
        }

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

        transfer_sol_from_user(get_rent_exempt(treasury)?, admin, treasury, system_program)?;

        Ok(())
    }
}
