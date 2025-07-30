use {
    crate::{
        error::CustomError,
        state::{
            AccountConfig, Bump, CommonConfig, SEED_ACCOUNT_CONFIG, SEED_BUMP, SEED_COMMON_CONFIG,
        },
        types::{AssetItem, Range},
    },
    anchor_lang::prelude::*,
    base::error::AuthError,
};

#[derive(Accounts)]
pub struct UpdateAccountConfig<'info> {
    pub sender: Signer<'info>,

    // data storage
    //
    #[account(
        seeds = [SEED_BUMP.as_bytes()],
        bump
    )]
    pub bump: Account<'info, Bump>,

    #[account(
        seeds = [SEED_COMMON_CONFIG.as_bytes()],
        bump = bump.common_config
    )]
    pub common_config: Account<'info, CommonConfig>,

    #[account(
        mut,
        seeds = [SEED_ACCOUNT_CONFIG.as_bytes()],
        bump = bump.account_config
    )]
    pub account_config: Account<'info, AccountConfig>,
}

impl<'info> UpdateAccountConfig<'info> {
    pub fn update_account_config(
        &mut self,
        registration_fee: Option<AssetItem>,
        data_size_range: Option<Range>,
        lifetime_range: Option<Range>,
        lifetime_margin_bps: Option<u16>,
    ) -> Result<()> {
        let UpdateAccountConfig {
            sender,
            common_config,
            account_config,
            ..
        } = self;

        if sender.key() != common_config.admin {
            Err(AuthError::Unauthorized)?;
        }

        let mut is_config_updated = false;

        if let Some(x) = registration_fee {
            // TODO: withdraw prev fee
            account_config.registration_fee = x;
            is_config_updated = true;
        }

        if let Some(x) = data_size_range {
            account_config.data_size_range = x;
            is_config_updated = true;
        }

        if let Some(x) = lifetime_range {
            account_config.lifetime_range = x;
            is_config_updated = true;
        }

        if let Some(x) = lifetime_margin_bps {
            account_config.lifetime_margin_bps = x;
            is_config_updated = true;
        }

        // don't allow empty instructions
        if !is_config_updated {
            Err(CustomError::NoParameters)?;
        }

        Ok(())
    }
}
