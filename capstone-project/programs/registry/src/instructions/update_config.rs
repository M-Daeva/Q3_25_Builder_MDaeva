use {
    crate::{
        error::CustomError,
        state::{Bump, Config, RotationState, SEED_ADMIN_ROTATION_STATE, SEED_BUMP, SEED_CONFIG},
        types::{AssetItem, Range},
    },
    anchor_lang::prelude::*,
    base::{error::AuthError, helpers::get_clock_time},
};

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    pub sender: Signer<'info>,

    // data storage
    //
    #[account(
        seeds = [SEED_BUMP.as_bytes()],
        bump
    )]
    pub bump: Account<'info, Bump>,

    #[account(
        mut,
        seeds = [SEED_CONFIG.as_bytes()],
        bump = bump.config
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [SEED_ADMIN_ROTATION_STATE.as_bytes()],
        bump = bump.rotation_state
    )]
    pub admin_rotation_state: Account<'info, RotationState>,
}

impl<'info> UpdateConfig<'info> {
    pub fn update_config(
        &mut self,
        admin: Option<Pubkey>,
        is_paused: Option<bool>,
        rotation_timeout: Option<u32>,
        registration_fee: Option<AssetItem>,
        data_size_range: Option<Range>,
    ) -> Result<()> {
        let UpdateConfig {
            sender,
            config,
            admin_rotation_state,
            ..
        } = self;

        if sender.key() != config.admin {
            Err(AuthError::Unauthorized)?;
        }

        let mut is_config_updated = false;

        if let Some(new_admin) = admin {
            admin_rotation_state.new_owner = Some(new_admin);
            admin_rotation_state.expiration_date =
                get_clock_time()? + config.rotation_timeout as u64;
            is_config_updated = true;
        }

        if let Some(x) = is_paused {
            config.is_paused = x;
            is_config_updated = true;
        }

        if let Some(x) = rotation_timeout {
            config.rotation_timeout = x;
            is_config_updated = true;
        }

        if let Some(x) = registration_fee {
            // TODO: withdraw prev fee
            config.registration_fee = x;
            is_config_updated = true;
        }

        if let Some(x) = data_size_range {
            config.data_size_range = x;
            is_config_updated = true;
        }

        // don't allow empty instructions
        if !is_config_updated {
            Err(CustomError::NoParameters)?;
        }

        Ok(())
    }
}
